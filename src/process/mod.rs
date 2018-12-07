mod git_update;
mod git_util;

use std::cell::Cell;
use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::option::Option;
use std::result::Result;
use std::thread;
use std::time::Duration;

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use jsonrpc_core;
use reqwest;
use serde_json;
use serde_json::Value;
use subprocess::{Exec, ExitStatus, Popen, PopenError, Redirection};

use super::rpc::types::NodeStatus;
use super::types::CommitHash;

#[derive(Debug)]
pub enum Error {
    EnvParseError,
    AlreadyRunning,
    NotRunning,
    // CodeChain is now updating, cannot run or stop CodeChain when updating
    Updating,
    SubprocessError(PopenError),
    IO(IOError),
    ShellError {
        exit_code: ExitStatus,
        stdout: String,
        stderr: String,
    },
    // This error caused when sending HTTP request to the codechain
    CodeChainRPC(String),
    Unknown(String),
}

impl From<PopenError> for Error {
    fn from(error: PopenError) -> Self {
        Error::SubprocessError(error)
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error)
    }
}

pub struct ProcessOption {
    pub codechain_dir: String,
    pub log_file_path: String,
}

enum CodeChainStatus {
    Starting {
        p2p_port: u16,
        rpc_port: u16,
    },
    Run {
        p2p_port: u16,
        rpc_port: u16,
    },
    Updating {
        env: String,
        args: String,
        sender: Cell<Option<git_update::Sender>>,
        rx_callback: Receiver<git_update::CallbackResult>,
    },
    Stop,
    Error {
        p2p_port: u16,
        rpc_port: u16,
    },
}

impl CodeChainStatus {
    fn to_node_status(&self) -> NodeStatus {
        match self {
            CodeChainStatus::Starting {
                ..
            } => NodeStatus::Starting,
            CodeChainStatus::Run {
                ..
            } => NodeStatus::Run,
            CodeChainStatus::Stop => NodeStatus::Stop,
            CodeChainStatus::Updating {
                ..
            } => NodeStatus::Updating,
            CodeChainStatus::Error {
                ..
            } => NodeStatus::Error,
        }
    }

    fn p2p_port(&self) -> Option<u16> {
        match self {
            CodeChainStatus::Starting {
                p2p_port,
                ..
            } => Some(*p2p_port),
            CodeChainStatus::Run {
                p2p_port,
                ..
            } => Some(*p2p_port),
            CodeChainStatus::Stop => None,
            CodeChainStatus::Updating {
                ..
            } => None,
            CodeChainStatus::Error {
                p2p_port,
                ..
            } => Some(*p2p_port),
        }
    }

    fn rpc_port(&self) -> u16 {
        match self {
            CodeChainStatus::Starting {
                rpc_port,
                ..
            } => *rpc_port,
            CodeChainStatus::Run {
                rpc_port,
                ..
            } => *rpc_port,
            CodeChainStatus::Stop => 0,
            CodeChainStatus::Updating {
                ..
            } => 0,
            CodeChainStatus::Error {
                rpc_port,
                ..
            } => *rpc_port,
        }
    }
}

pub struct Process {
    option: ProcessOption,
    child: Option<Popen>,
    codechain_status: CodeChainStatus,
    http_client: reqwest::Client,
}

type Callback<T> = Sender<Result<T, Error>>;

pub enum Message {
    Run {
        env: String,
        args: String,
        callback: Callback<()>,
    },
    Stop {
        callback: Callback<()>,
    },
    #[allow(dead_code)]
    Quit {
        callback: Callback<()>,
    },
    Update {
        env: String,
        args: String,
        target_version: CommitHash,
        callback: Callback<()>,
    },
    GetStatus {
        callback: Callback<(NodeStatus, Option<u16>, CommitHash)>,
    },
    GetLog {
        callback: Callback<String>,
    },
    CallRPC {
        method: String,
        arguments: Vec<Value>,
        callback: Callback<Value>,
    },
}

impl Process {
    pub fn run_thread(option: ProcessOption) -> Sender<Message> {
        let mut process = Self {
            option,
            child: None,
            codechain_status: CodeChainStatus::Stop,
            http_client: reqwest::Client::new(),
        };
        let (tx, rx) = channel::unbounded();
        thread::Builder::new()
            .name("process".to_string())
            .spawn(move || loop {
                let timeout = Duration::new(1, 0);
                let message = select! {
                    recv(rx, message) => {
                        message
                    },
                    recv(channel::after(timeout)) => {
                        None
                    }
                };
                if let Some(m) = message {
                    process.handle_message(m);
                }
                process.ping_to_codechain();
                process.handle_git_update();
            })
            .expect("Should success running process thread");
        tx
    }

    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Run {
                env,
                args,
                callback,
            } => {
                let result = self.run(&env, &args);
                callback.send(result);
            }
            Message::Stop {
                callback,
            } => {
                let result = self.stop();
                callback.send(result);
            }
            Message::Quit {
                callback,
            } => {
                let result = self.stop();
                if let CodeChainStatus::Updating {
                    sender,
                    ..
                } = &mut self.codechain_status
                {
                    cinfo!(PROCESS, "Wait until codechain update finish");
                    let moved_sender = sender.replace(None).expect("Sender should be exist");
                    if let Err(err) = moved_sender.join() {
                        cerror!(PROCESS, "Cannot wait for git update closing: {:?}", err);
                    }
                }
                callback.send(result);
                return
            }
            Message::Update {
                env,
                args,
                target_version,
                callback,
            } => {
                let result = if self.check_running() {
                    self.stop()
                } else {
                    Ok(())
                };
                let result = result.and_then(|_| self.update(&target_version, env, args));
                callback.send(result);
            }
            Message::GetStatus {
                callback,
            } => {
                let codechain_status = &self.codechain_status;
                let status = codechain_status.to_node_status();
                let p2p_port = codechain_status.p2p_port();
                let commit_hash = self.get_commit_hash().unwrap_or_default();
                callback.send(Ok((status, p2p_port, commit_hash)));
            }
            Message::GetLog {
                callback,
            } => {
                let result = self.get_log();
                callback.send(result);
            }
            Message::CallRPC {
                method,
                arguments,
                callback,
            } => {
                let result = self.call_rpc(method, arguments);
                callback.send(result);
            }
        }
    }

    pub fn ping_to_codechain(&mut self) {
        if let CodeChainStatus::Stop = self.codechain_status {
            return
        }

        if let CodeChainStatus::Updating {
            ..
        } = self.codechain_status
        {
            return
        }

        ctrace!(PROCESS, "Ping to CodeChain");

        let result = self.call_rpc("ping".to_string(), Vec::new());
        ctrace!(PROCESS, "{:?}", result);

        match self.codechain_status {
            CodeChainStatus::Run {
                p2p_port,
                rpc_port,
            } => {
                if let Err(err) = result {
                    cinfo!(PROCESS, "Codechain ping error {:#?}", err);
                    self.codechain_status = CodeChainStatus::Error {
                        p2p_port,
                        rpc_port,
                    };
                }
            }
            CodeChainStatus::Starting {
                p2p_port,
                rpc_port,
            } => {
                if result.is_ok() {
                    cinfo!(PROCESS, "CodeChain is running now");
                    self.codechain_status = CodeChainStatus::Run {
                        p2p_port,
                        rpc_port,
                    };
                }
                if !self.check_running() {
                    self.codechain_status = CodeChainStatus::Error {
                        p2p_port,
                        rpc_port,
                    };
                }
            }
            CodeChainStatus::Stop => {
                cerror!(PROCESS, "Should not reach here");
            }
            CodeChainStatus::Error {
                p2p_port,
                rpc_port,
            } => {
                if result.is_ok() {
                    cinfo!(PROCESS, "CodeChain comback to normal");
                    self.codechain_status = CodeChainStatus::Run {
                        p2p_port,
                        rpc_port,
                    };
                }
            }
            CodeChainStatus::Updating {
                ..
            } => unreachable!(),
        }
    }

    fn handle_git_update(&mut self) {
        let (success, env, args) = if let CodeChainStatus::Updating {
            rx_callback,
            env,
            args,
            ..
        } = &self.codechain_status
        {
            match rx_callback.try_recv() {
                None => return,
                Some(Err(err)) => {
                    cinfo!(PROCESS, "Git update update failed : {:?}", err);
                    (false, env.to_string(), args.to_string())
                }
                Some(Ok(_)) => {
                    cinfo!(PROCESS, "Git update success");
                    (true, env.to_string(), args.to_string())
                }
            }
        } else {
            return
        };

        if success {
            self.codechain_status = CodeChainStatus::Stop;
            if let Err(err) = self.run(&env, &args) {
                cerror!(PROCESS, "Cannot run codechain after update : {:?}", err);
            }
        } else {
            self.codechain_status = CodeChainStatus::Error {
                p2p_port: 0,
                rpc_port: 0,
            };
        }
    }

    pub fn run(&mut self, env: &str, args: &str) -> Result<(), Error> {
        cdebug!(PROCESS, "Run codechain");
        if self.check_running() {
            cdebug!(PROCESS, "Run codechain failed because it is AlreadyRunning");
            return Err(Error::AlreadyRunning)
        }
        if self.is_updating() {
            cdebug!(PROCESS, "Run codechain failed because it is Updating");
            return Err(Error::Updating)
        }

        let args_iter = args.split_whitespace();
        let args_vec: Vec<String> = args_iter.map(|str| str.to_string()).collect();

        let (p2p_port, rpc_port) = parse_ports(&args_vec);

        let envs = Self::parse_env(env)?;

        let file = File::create(self.option.log_file_path.clone())?;

        let mut exec = Exec::cmd("cargo")
            .arg("run")
            .arg("--")
            .cwd(self.option.codechain_dir.clone())
            .stdout(Redirection::File(file))
            .stderr(Redirection::Merge)
            .args(&args_vec);

        for (k, v) in envs {
            exec = exec.env(k, v);
        }

        let child = exec.popen()?;
        self.child = Some(child);

        self.codechain_status = CodeChainStatus::Starting {
            p2p_port,
            rpc_port,
        };

        Ok(())
    }

    pub fn check_running(&mut self) -> bool {
        self.child.as_mut().map_or(false, |child| child.poll().is_none())
    }

    fn is_updating(&self) -> bool {
        if let CodeChainStatus::Updating {
            ..
        } = self.codechain_status
        {
            true
        } else {
            false
        }
    }

    fn parse_env(env: &str) -> Result<Vec<(&str, &str)>, Error> {
        let env_kvs = env.split_whitespace();
        let mut ret = Vec::new();
        for env_kv in env_kvs {
            let kv_array: Vec<&str> = env_kv.splitn(2, '=').collect();
            if kv_array.len() != 2 {
                return Err(Error::EnvParseError)
            } else {
                ret.push((kv_array[0], kv_array[1]));
            }
        }
        Ok(ret)
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        if !self.check_running() {
            return Err(Error::NotRunning)
        }
        if self.is_updating() {
            return Err(Error::Updating)
        }

        let codechain = &mut self.child.as_mut().expect("Already checked");
        ctrace!(PROCESS, "Send SIGTERM to CodeChain");
        codechain.terminate()?;

        let wait_result = codechain.wait_timeout(Duration::new(10, 0))?;

        if let Some(exit_code) = wait_result {
            ctrace!(PROCESS, "CodeChain closed with {:?}", exit_code);
            self.codechain_status = CodeChainStatus::Stop;
            return Ok(())
        }

        cinfo!(PROCESS, "CodeChain does not exit after 10 seconds");

        codechain.kill()?;

        self.codechain_status = CodeChainStatus::Stop;

        Ok(())
    }

    fn update(&mut self, commit_hash: &str, env: String, args: String) -> Result<(), Error> {
        if self.is_updating() {
            return Err(Error::Updating)
        }

        let (tx, rx) = channel::unbounded();
        let job_sender = git_update::Job::run(self.option.codechain_dir.to_string(), commit_hash.to_string(), tx);

        self.codechain_status = CodeChainStatus::Updating {
            env,
            args,
            sender: Cell::new(Some(job_sender)),
            rx_callback: rx,
        };

        Ok(())
    }

    fn get_log(&mut self) -> Result<String, Error> {
        let file_name = self.option.log_file_path.clone();
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn call_rpc(&self, method: String, arguments: Vec<Value>) -> Result<Value, Error> {
        let params = jsonrpc_core::Params::Array(arguments);

        let jsonrpc_request = jsonrpc_core::MethodCall {
            jsonrpc: None,
            method,
            params: Some(params),
            id: jsonrpc_core::Id::Num(1),
        };

        ctrace!(PROCESS, "Send JSONRPC to CodeChain {:#?}", jsonrpc_request);

        let url = format!("http://127.0.0.1:{}/", self.codechain_status.rpc_port());
        let mut response = self
            .http_client
            .post(&url)
            .json(&jsonrpc_request)
            .send()
            .map_err(|err| Error::CodeChainRPC(format!("{}", err)))?;

        let response: jsonrpc_core::Response =
            response.json().map_err(|err| Error::CodeChainRPC(format!("JSON parse failed {}", err)))?;
        ctrace!(PROCESS, "Recieve JSONRPC response from CodeChain {:#?}", response);
        let value = serde_json::to_value(response).expect("Should success jsonrpc type to Value");

        Ok(value)
    }

    fn get_commit_hash(&self) -> Result<String, Error> {
        if let CodeChainStatus::Run {
            ..
        } = self.codechain_status
        {
            let response = self.call_rpc("commitHash".to_string(), Vec::new())?;
            Ok(response["result"].as_str().unwrap_or("").to_string())
        } else {
            Ok(git_util::current_hash(self.option.codechain_dir.clone())?)
        }
    }
}

fn parse_ports(args: &[String]) -> (u16, u16) {
    let p2p_port = parse_port(args, "--port");
    let rpc_port = parse_port(args, "--jsonrpc-port");

    (p2p_port.unwrap_or(3485), rpc_port.unwrap_or(8080))
}

fn parse_port(args: &[String], option_name: &str) -> Option<u16> {
    let option_position = args.iter().position(|arg| arg == option_name);
    let interface_pos = option_position.map(|pos| pos + 1);
    let interface_string = interface_pos.and_then(|pos| args.get(pos));
    interface_string.and_then(|port| port.parse().ok())
}
