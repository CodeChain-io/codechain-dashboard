mod git_update;

use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::option::Option;
use std::result::Result;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

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

#[derive(Debug, PartialEq, Clone)]
enum CodeChainStatus {
    Starting {
        p2p_port: u16,
        rpc_port: u16,
    },
    Run {
        p2p_port: u16,
        rpc_port: u16,
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
            CodeChainStatus::Error {
                rpc_port,
                ..
            } => *rpc_port,
        }
    }
}

pub struct Process {
    option: ProcessOption,
    // first element is CodeChain second element is `tee` command
    child: Option<Vec<Popen>>,
    codechain_status: CodeChainStatus,
}

pub enum Message {
    Run {
        env: String,
        args: String,
        callback: Sender<Result<(), Error>>,
    },
    Stop {
        callback: Sender<Result<(), Error>>,
    },
    Quit {
        callback: Sender<Result<(), Error>>,
    },
    Update {
        env: String,
        args: String,
        target_version: CommitHash,
        callback: Sender<Result<(), Error>>,
    },
    GetStatus {
        callback: Sender<Result<(NodeStatus, Option<u16>, CommitHash), Error>>,
    },
    GetLog {
        callback: Sender<Result<String, Error>>,
    },
    CallRPC {
        method: String,
        arguments: Vec<Value>,
        callback: Sender<Result<Value, Error>>,
    },
}

impl Process {
    pub fn run_thread(option: ProcessOption) -> Sender<Message> {
        let mut process = Self {
            option,
            child: None,
            codechain_status: CodeChainStatus::Stop,
        };
        let (tx, rx) = channel();
        thread::Builder::new()
            .name("process".to_string())
            .spawn(move || loop {
                let message = match rx.recv_timeout(Duration::new(1, 0)) {
                    Ok(message) => Some(message),
                    Err(RecvTimeoutError::Timeout) => None,
                    Err(RecvTimeoutError::Disconnected) => {
                        cwarn!(PROCESS, "Process's sender has disconnected");
                        None
                    }
                };
                if let Some(message) = message {
                    process.handle_message(message);
                }
                process.ping_to_codechain();
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
                let result = self.run(env, args);
                callback.send(result).expect("Callback should be success");
            }
            Message::Stop {
                callback,
            } => {
                let result = self.stop();
                callback.send(result).expect("Callback should be success");
            }
            Message::Quit {
                callback,
            } => {
                let result = self.stop();
                callback.send(result).expect("Callback should be success");
                return
            }
            Message::Update {
                env,
                args,
                target_version,
                callback,
            } => {
                let mut result = Ok(());
                if self.is_running() {
                    result = self.stop();
                }
                let result = result.and_then(|_| self.update(&target_version));
                let result = result.and_then(|_| self.run(env, args));
                callback.send(result).expect("Callback should be success");
            }
            Message::GetStatus {
                callback,
            } => {
                let codechain_status = &self.codechain_status;
                let status = codechain_status.to_node_status();
                let p2p_port = codechain_status.p2p_port();
                let commit_hash = self.get_commit_hash().unwrap_or("".to_string());
                callback.send(Ok((status, p2p_port, commit_hash))).expect("Callback should be success");
            }
            Message::GetLog {
                callback,
            } => {
                let result = self.get_log();
                callback.send(result).expect("Callback should be success");
            }
            Message::CallRPC {
                method,
                arguments,
                callback,
            } => {
                let result = self.call_rpc(method, arguments);
                callback.send(result).expect("Callback should be success")
            }
        }
    }

    pub fn ping_to_codechain(&mut self) {
        if self.codechain_status == CodeChainStatus::Stop {
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
                if !self.is_running() {
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
                cinfo!(PROCESS, "CodeChain comabck to normal");
                self.codechain_status = CodeChainStatus::Run {
                    p2p_port,
                    rpc_port,
                };
            }
        }
    }

    pub fn run(&mut self, env: String, args: String) -> Result<(), Error> {
        cdebug!(PROCESS, "Run codechain");
        if self.is_running() {
            cdebug!(PROCESS, "Run codechain failed because it is AlreadyRunning");
            return Err(Error::AlreadyRunning)
        }

        let args_iter = args.split_whitespace();
        let args_vec: Vec<String> = args_iter.map(|str| str.to_string()).collect();

        let (p2p_port, rpc_port) = parse_ports(&args_vec);

        let envs = Self::parse_env(&env)?;

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
        self.child = Some(vec![child]);

        self.codechain_status = CodeChainStatus::Starting {
            p2p_port,
            rpc_port,
        };

        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        if self.child.is_none() {
            return false
        }

        let child = self.child.as_mut().unwrap();
        if child[0].poll().is_none() {
            return true
        } else {
            return false
        }
    }

    fn parse_env(env: &str) -> Result<Vec<(&str, &str)>, Error> {
        let env_kvs = env.split_whitespace();
        let mut ret = Vec::new();
        for env_kv in env_kvs {
            let kv_array: Vec<&str> = env_kv.splitn(2, "=").collect();
            if kv_array.len() != 2 {
                return Err(Error::EnvParseError)
            } else {
                ret.push((kv_array[0], kv_array[1]));
            }
        }
        return Ok(ret)
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        if !self.is_running() {
            return Err(Error::NotRunning)
        }

        let codechain = &mut self.child.as_mut().expect("Already checked")[0];
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

    fn update(&mut self, commit_hash: &str) -> Result<(), Error> {
        git_remote_update(self.option.codechain_dir.clone())?;
        git_reset_hard(self.option.codechain_dir.clone(), commit_hash.to_string())?;
        let current_hash = git_current_hash(self.option.codechain_dir.clone())?;
        if commit_hash != current_hash {
            cwarn!(PROCESS, "Updated commit hash not matched expected {} found {}", commit_hash, current_hash);
            Err(Error::Unknown(format!("Cannot update to {}", commit_hash)))
        } else {
            Ok(())
        }
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
        let client = reqwest::Client::new();
        let mut response =
            client.post(&url).json(&jsonrpc_request).send().map_err(|err| Error::CodeChainRPC(format!("{}", err)))?;

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
            Ok(git_current_hash(self.option.codechain_dir.clone())?)
        }
    }
}

fn parse_ports(args: &Vec<String>) -> (u16, u16) {
    let p2p_port = parse_port(args, "--port");
    let rpc_port = parse_port(args, "--jsonrpc-port");

    (p2p_port.unwrap_or(3485), rpc_port.unwrap_or(8080))
}

fn parse_port(args: &Vec<String>, option_name: &str) -> Option<u16> {
    let option_position = args.iter().position(|arg| arg == option_name);
    let interface_pos = option_position.map(|pos| pos + 1);
    let interface_string = interface_pos.and_then(|pos| args.get(pos));
    interface_string.and_then(|port| port.parse().ok())
}

fn git_remote_update(codechain_dir: String) -> Result<(), Error> {
    cinfo!(PROCESS, "Run git remote update");
    let exec = Exec::cmd("git").arg("remote").arg("update").cwd(codechain_dir).capture()?;
    if exec.exit_status.success() {
        ctrace!(PROCESS, "git remote update\n  stdout: {}\n  stderr: {}\n", exec.stdout_str(), exec.stderr_str());
        Ok(())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}

fn git_reset_hard(codechain_dir: String, target_commit_hash: CommitHash) -> Result<(), Error> {
    cinfo!(PROCESS, "Run git reset --hard");
    let exec = Exec::cmd("git").arg("reset").arg("--hard").arg(target_commit_hash).cwd(codechain_dir).capture()?;
    if exec.exit_status.success() {
        ctrace!(PROCESS, "git remote update\n  stdout: {}\n  stderr: {}\n", exec.stdout_str(), exec.stderr_str());
        Ok(())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}

fn git_current_hash(codechain_dir: String) -> Result<CommitHash, Error> {
    cdebug!(PROCESS, "Run git rev-parse HEAD at {}", codechain_dir);
    let exec = Exec::cmd("git").arg("rev-parse").arg("HEAD").cwd(codechain_dir).capture()?;
    Ok(exec.stdout_str().trim().to_string())
}
