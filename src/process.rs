use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::net::SocketAddr;
use std::option::Option;
use std::result::Result;
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use jsonrpc_core;
use reqwest;
use serde_json;
use serde_json::Value;
use subprocess::{Exec, Popen, PopenError, Redirection};

use super::rpc::types::NodeStatus;

#[derive(Debug)]
pub enum Error {
    EnvParseError,
    AlreadyRunning,
    NotRunning,
    SubprocessError(PopenError),
    IO(IOError),
    // This error caused when sending HTTP request to the codechain
    CodeChainRPC(String),
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

    fn p2p_port(&self) -> u16 {
        match self {
            CodeChainStatus::Starting {
                p2p_port,
                ..
            } => *p2p_port,
            CodeChainStatus::Run {
                p2p_port,
                ..
            } => *p2p_port,
            CodeChainStatus::Stop => 0,
            CodeChainStatus::Error {
                p2p_port,
                ..
            } => *p2p_port,
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
    GetStatus {
        callback: Sender<Result<(NodeStatus, u16), Error>>,
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
                        cwarn!("Process's sender has disconnected");
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
                callback.send(Ok(())).expect("Callback should be success");
                return
            }
            Message::GetStatus {
                callback,
            } => {
                let codechain_status = &self.codechain_status;
                let status = codechain_status.to_node_status();
                let p2p_port = codechain_status.p2p_port();
                callback.send(Ok((status, p2p_port))).expect("Callback should be success");
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

        ctrace!("Ping to CodeChain");

        let result = self.call_rpc("ping".to_string(), Vec::new());
        cinfo!("{:?}", result);

        match self.codechain_status {
            CodeChainStatus::Run {
                p2p_port,
                rpc_port,
            } => {
                if let Err(err) = result {
                    cinfo!("Codechain ping error {:#?}", err);
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
                    cinfo!("CodeChain is running now");
                    self.codechain_status = CodeChainStatus::Run {
                        p2p_port,
                        rpc_port,
                    };
                }
            }
            CodeChainStatus::Stop => {
                cerror!("Should not reach here");
            }
            CodeChainStatus::Error {
                p2p_port,
                rpc_port,
            } => {
                cinfo!("CodeChain comabck to normal");
                self.codechain_status = CodeChainStatus::Run {
                    p2p_port,
                    rpc_port,
                };
            }
        }
    }

    pub fn run(&mut self, env: String, args: String) -> Result<(), Error> {
        if self.is_running() {
            return Err(Error::AlreadyRunning)
        }

        let args_iter = args.split_whitespace();
        let args_vec: Vec<String> = args_iter.map(|str| str.to_string()).collect();

        let (p2p_port, rpc_port) = parse_ports(&args_vec);

        let envs = Self::parse_env(&env)?;

        let mut exec = Exec::cmd("cargo")
            .arg("run")
            .arg("--")
            .cwd(self.option.codechain_dir.clone())
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .args(&args_vec);

        for (k, v) in envs {
            exec = exec.env(k, v);
        }

        let child = (exec | Exec::cmd("tee").arg(self.option.log_file_path.clone())).popen()?;
        self.child = Some(child);

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
        ctrace!("Send SIGTERM to CodeChain");
        codechain.terminate()?;

        let wait_result = codechain.wait_timeout(Duration::new(10, 0))?;

        if let Some(exit_code) = wait_result {
            ctrace!("CodeChain closed with {:?}", exit_code);
            self.codechain_status = CodeChainStatus::Stop;
            return Ok(())
        }

        cinfo!("CodeChain does not exit after 10 seconds");

        codechain.kill()?;

        self.codechain_status = CodeChainStatus::Stop;

        Ok(())
    }

    fn get_log(&mut self) -> Result<String, Error> {
        let file_name = self.option.log_file_path.clone();
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn call_rpc(&mut self, method: String, arguments: Vec<Value>) -> Result<Value, Error> {
        // FIXME: Get port number from args
        let port = 8080;

        let params = jsonrpc_core::Params::Array(arguments);

        let jsonrpc_request = jsonrpc_core::MethodCall {
            jsonrpc: None,
            method,
            params: Some(params),
            id: jsonrpc_core::Id::Num(1),
        };

        let url = format!("http://127.0.0.1:{}/", port);
        let client = reqwest::Client::new();
        let mut response =
            client.post(&url).json(&jsonrpc_request).send().map_err(|err| Error::CodeChainRPC(format!("{}", err)))?;

        let response: jsonrpc_core::Response =
            response.json().map_err(|err| Error::CodeChainRPC(format!("JSON parse failed {}", err)))?;
        let value = serde_json::to_value(response).expect("Should success jsonrpc type to Value");

        Ok(value)
    }
}

fn parse_ports(args: &Vec<String>) -> (u16, u16) {
    let p2p_port = parse_port(args, "interface", 3485);
    let rpc_port = parse_port(args, "jsonrpc-interface", 8080);

    (p2p_port, rpc_port)
}

fn parse_port(args: &Vec<String>, option_name: &str, default_port: u16) -> u16 {
    let option_position = args.iter().position(|arg| arg == option_name);
    let interface_pos = option_position.map(|pos| pos + 1);
    let interface_string = interface_pos.and_then(|pos| args.get(pos));
    let interface: Option<SocketAddr> = interface_string.and_then(|port| port.parse().ok());
    interface.map(|interface| interface.port()).unwrap_or(default_port)
}
