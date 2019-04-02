mod binary_update;
mod codechain_process;
mod fs_util;
mod git_update;
mod git_util;
mod rpc;
mod update;

use std::cell::Cell;
use std::io::Error as IOError;
use std::option::Option;
use std::result::Result;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use parking_lot::Mutex;
use serde_json::Value;
use subprocess::{Exec, ExitStatus, PopenError};

use self::codechain_process::CodeChainProcess;
use super::rpc::types::{NodeStatus, UpdateCodeChainRequest};
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
    BinaryChecksumMismatch {
        expected: String,
        actual: String,
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
        rpc_client: rpc::RPCClient,
    },
    Run {
        p2p_port: u16,
        rpc_client: rpc::RPCClient,
    },
    Updating {
        env: String,
        args: String,
        sender: Cell<Option<update::Sender>>,
        rx_callback: Receiver<update::CallbackResult>,
    },
    Stop,
    Error {
        p2p_port: u16,
        rpc_client: Option<rpc::RPCClient>,
    },
    Temp,
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
            CodeChainStatus::Temp => unreachable!(),
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
            CodeChainStatus::Temp => unreachable!(),
        }
    }

    fn rpc_client(&self) -> Option<&rpc::RPCClient> {
        match self {
            CodeChainStatus::Starting {
                rpc_client,
                ..
            } => Some(rpc_client),
            CodeChainStatus::Run {
                rpc_client,
                ..
            } => Some(rpc_client),
            CodeChainStatus::Stop => None,
            CodeChainStatus::Updating {
                ..
            } => None,
            CodeChainStatus::Error {
                rpc_client,
                ..
            } => rpc_client.as_ref(),
            CodeChainStatus::Temp => unreachable!(),
        }
    }
}

pub struct Process {
    option: ProcessOption,
    child: Option<CodeChainProcess>,
}

type Callback<T> = Sender<Result<T, Error>>;

pub struct ProcessGetStatusResult {
    pub status: NodeStatus,
    pub port: Option<u16>,
    pub commit_hash: CommitHash,
    pub binary_checksum: String,
}

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
        target: UpdateCodeChainRequest,
        callback: Callback<()>,
    },
    GetStatus {
        callback: Callback<ProcessGetStatusResult>,
    },
    GetLog {
        levels: Vec<String>,
        callback: Callback<Vec<Value>>,
    },
    CallRPC {
        method: String,
        arguments: Vec<Value>,
        callback: Callback<Value>,
    },
}

impl Process {
    pub fn run_thread(option: ProcessOption) -> Sender<Message> {
        let codechain_status: Arc<Mutex<CodeChainStatus>> = Arc::new(Mutex::new(CodeChainStatus::Stop));
        let mut process = Self {
            option,
            child: None,
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
                    process.handle_message(m, codechain_status.as_ref());
                }
                process.ping_to_codechain(codechain_status.as_ref());
                process.handle_update(codechain_status.as_ref());
            })
            .expect("Should success running process thread");
        tx
    }

    fn handle_message(&mut self, message: Message, codechain_status: &Mutex<CodeChainStatus>) {
        match message {
            Message::Run {
                env,
                args,
                callback,
            } => {
                let result = self.run(&env, &args, &mut codechain_status.lock());
                callback.send(result);
            }
            Message::Stop {
                callback,
            } => {
                let result = self.stop(&mut *codechain_status.lock());
                callback.send(result);
            }
            Message::Quit {
                callback,
            } => {
                let result = self.stop(&mut *codechain_status.lock());
                if let CodeChainStatus::Updating {
                    sender,
                    ..
                } = &*codechain_status.lock()
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
                target,
                callback,
            } => {
                let result = if self.check_running() {
                    self.stop(&mut *codechain_status.lock())
                } else {
                    Ok(())
                };
                let result = result.and_then(|_| self.update(&target, env, args, &mut *codechain_status.lock()));
                callback.send(result);
            }
            Message::GetStatus {
                callback,
            } => {
                let codechain_status = codechain_status.lock();
                let status = codechain_status.to_node_status();
                let p2p_port = codechain_status.p2p_port();
                let commit_hash = self.get_commit_hash(&*codechain_status).unwrap_or_default();
                let binary_checksum =
                    fs_util::get_checksum_or_default(&self.option.codechain_dir, "codechain").unwrap_or_default();
                callback.send(Ok(ProcessGetStatusResult {
                    status,
                    port: p2p_port,
                    commit_hash,
                    binary_checksum,
                }));
            }
            Message::GetLog {
                levels,
                callback,
            } => {
                let result = self.get_log(levels, &*codechain_status.lock());
                callback.send(result);
            }
            Message::CallRPC {
                method,
                arguments,
                callback,
            } => match codechain_status.lock().rpc_client() {
                Some(rpc_client) => {
                    let result =
                        rpc_client.call_rpc(method, arguments).map_err(|err| Error::CodeChainRPC(err.to_string()));
                    callback.send(result)
                }
                None => callback.send(Err(Error::NotRunning)),
            },
        }
    }

    fn ping_to_codechain(&mut self, codechain_status: &Mutex<CodeChainStatus>) {
        let mut codechain_status = codechain_status.lock();
        if let CodeChainStatus::Stop = *codechain_status {
            return
        }

        if let CodeChainStatus::Updating {
            ..
        } = *codechain_status
        {
            return
        }

        ctrace!(PROCESS, "Ping to CodeChain");

        let result = match codechain_status.rpc_client() {
            Some(rpc_client) => Some(rpc_client.call_rpc("ping".to_string(), Vec::new())),
            None => None,
        };
        ctrace!(PROCESS, "{:?}", result);

        let original: CodeChainStatus = ::std::mem::replace(&mut *codechain_status, CodeChainStatus::Temp);
        let next_status: CodeChainStatus = match original {
            CodeChainStatus::Run {
                p2p_port,
                rpc_client,
            } => {
                if let Err(err) = result.unwrap() {
                    cinfo!(PROCESS, "Codechain ping error {:#?}", err);
                    CodeChainStatus::Error {
                        p2p_port,
                        rpc_client: Some(rpc_client),
                    }
                } else {
                    CodeChainStatus::Run {
                        p2p_port,
                        rpc_client,
                    }
                }
            }
            CodeChainStatus::Starting {
                p2p_port,
                rpc_client,
            } => {
                if result.unwrap().is_ok() {
                    cinfo!(PROCESS, "CodeChain is running now");
                    CodeChainStatus::Run {
                        p2p_port,
                        rpc_client,
                    }
                } else if !self.check_running() {
                    CodeChainStatus::Error {
                        p2p_port,
                        rpc_client: Some(rpc_client),
                    }
                } else {
                    CodeChainStatus::Starting {
                        p2p_port,
                        rpc_client,
                    }
                }
            }
            CodeChainStatus::Stop => unreachable!(),
            CodeChainStatus::Error {
                p2p_port,
                rpc_client,
            } => {
                if let Some(Ok(_)) = result {
                    cinfo!(PROCESS, "CodeChain returned to normal");
                    CodeChainStatus::Run {
                        p2p_port,
                        rpc_client: rpc_client.unwrap(),
                    }
                } else {
                    CodeChainStatus::Error {
                        p2p_port,
                        rpc_client,
                    }
                }
            }
            CodeChainStatus::Updating {
                ..
            } => unreachable!(),
            CodeChainStatus::Temp => unreachable!(),
        };

        *codechain_status = next_status;
    }

    fn handle_update(&mut self, codechain_status: &Mutex<CodeChainStatus>) {
        let mut codechain_status = codechain_status.lock();
        let (success, env, args) = if let CodeChainStatus::Updating {
            rx_callback,
            env,
            args,
            ..
        } = &*codechain_status
        {
            match rx_callback.try_recv() {
                None => return,
                Some(Err(err)) => {
                    cinfo!(PROCESS, "Update failed : {:?}", err);
                    (false, env.to_string(), args.to_string())
                }
                Some(Ok(_)) => {
                    cinfo!(PROCESS, "Update success");
                    (true, env.to_string(), args.to_string())
                }
            }
        } else {
            return
        };

        if success {
            *codechain_status = CodeChainStatus::Stop;
            if let Err(err) = self.run(&env, &args, &mut codechain_status) {
                cerror!(PROCESS, "Cannot run codechain after update : {:?}", err);
            }
        } else {
            *codechain_status = CodeChainStatus::Error {
                p2p_port: 0,
                rpc_client: None,
            };
        }
    }

    fn run(&mut self, env: &str, args: &str, codechain_status: &mut CodeChainStatus) -> Result<(), Error> {
        cinfo!(PROCESS, "Run codechain");
        if self.check_running() {
            cinfo!(PROCESS, "Run codechain failed because it is AlreadyRunning");
            return Err(Error::AlreadyRunning)
        }
        if self.is_updating(codechain_status) {
            cinfo!(PROCESS, "Run codechain failed because it is Updating");
            return Err(Error::Updating)
        }

        let args_iter = args.split_whitespace();
        let args_vec: Vec<String> = args_iter.map(|str| str.to_string()).collect();
        let (p2p_port, rpc_port) = parse_ports(&args_vec);
        let envs = Self::parse_env(env)?;

        let child = CodeChainProcess::new(envs, args_vec, &self.option).map_err(Error::Unknown)?;
        self.child = Some(child);

        *codechain_status = CodeChainStatus::Starting {
            p2p_port,
            rpc_client: rpc::RPCClient::new(rpc_port),
        };

        Ok(())
    }

    pub fn check_running(&mut self) -> bool {
        self.child.as_ref().map_or(false, |child| child.is_running())
    }

    fn is_updating(&self, codechain_status: &CodeChainStatus) -> bool {
        if let CodeChainStatus::Updating {
            ..
        } = codechain_status
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

    fn stop(&mut self, codechain_status: &mut CodeChainStatus) -> Result<(), Error> {
        if !self.check_running() {
            return Err(Error::NotRunning)
        }
        if self.is_updating(codechain_status) {
            return Err(Error::Updating)
        }

        cinfo!(PROCESS, "Stop CodeChain");

        let codechain = self.child.as_ref().expect("Already checked");
        ctrace!(PROCESS, "Send SIGTERM to CodeChain");
        codechain.terminate()?;

        let wait_result = codechain.wait_timeout(Duration::new(10, 0))?;

        if let Some(exit_code) = wait_result {
            ctrace!(PROCESS, "CodeChain closed with {:?}", exit_code);
            *codechain_status = CodeChainStatus::Stop;
            return Ok(())
        }

        cinfo!(PROCESS, "CodeChain does not exit after 10 seconds");

        codechain.kill()?;

        *codechain_status = CodeChainStatus::Stop;

        Ok(())
    }

    fn update(
        &mut self,
        target: &UpdateCodeChainRequest,
        env: String,
        args: String,
        codechain_status: &mut CodeChainStatus,
    ) -> Result<(), Error> {
        if self.is_updating(codechain_status) {
            return Err(Error::Updating)
        }

        cinfo!(PROCESS, "Update CodeChain");

        let (tx, rx) = channel::unbounded();
        let job_sender = match target {
            UpdateCodeChainRequest::Git {
                commit_hash,
            } => git_update::Job::run(self.option.codechain_dir.to_string(), commit_hash.to_string(), tx),
            UpdateCodeChainRequest::Binary {
                binary_url,
                binary_checksum,
            } => binary_update::Job::run(
                self.option.codechain_dir.to_string(),
                binary_url.to_string(),
                binary_checksum.to_string(),
                tx,
            ),
        };

        *codechain_status = CodeChainStatus::Updating {
            env,
            args,
            sender: Cell::new(Some(job_sender)),
            rx_callback: rx,
        };

        Ok(())
    }

    fn get_log(&mut self, levels: Vec<String>, codechain_status: &CodeChainStatus) -> Result<Vec<Value>, Error> {
        let rpc_client = match codechain_status.rpc_client() {
            Some(rpc_client) => rpc_client,
            None => return Err(Error::NotRunning),
        };
        let mut response =
            rpc_client.call_rpc("slog".to_string(), Vec::new()).map_err(|err| Error::CodeChainRPC(err.to_string()))?;
        let result = response
            .get_mut("result")
            .ok_or_else(|| Error::CodeChainRPC("JSON parse failed: cannot find the result field".to_string()))?;
        let logs = result
            .as_array_mut()
            .ok_or_else(|| Error::CodeChainRPC("JSON parse failed: slog's result is not array".to_string()))?;

        let empty_string = Value::String("".to_string());
        let filtered_logs = logs
            .iter_mut()
            .filter(|log| {
                let target = log.pointer("/level").unwrap_or(&empty_string).as_str().unwrap_or("");
                levels.iter().any(|t| target.to_lowercase() == t.to_lowercase())
            })
            .map(|value| value.take());

        Ok(filtered_logs.collect())
    }

    fn get_commit_hash(&self, codechain_status: &CodeChainStatus) -> Result<String, Error> {
        if let CodeChainStatus::Run {
            rpc_client,
            ..
        } = &codechain_status
        {
            let response = rpc_client
                .call_rpc("commitHash".to_string(), Vec::new())
                .map_err(|err| Error::CodeChainRPC(err.to_string()))?;
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
