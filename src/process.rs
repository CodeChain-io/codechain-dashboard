use std::option::Option;
use std::result::Result;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use subprocess::{Exec, Popen, PopenError, Redirection};

pub enum Error {
    EnvParseError,
    AlreadyRunning,
    SubprocessError(PopenError),
}

impl From<PopenError> for Error {
    fn from(error: PopenError) -> Self {
        Error::SubprocessError(error)
    }
}

pub struct ProcessOption {
    pub codechain_dir: String,
    pub log_file_path: String,
}

pub struct Process {
    option: ProcessOption,
    // first element is CodeChain second element is `tee` command
    child: Option<Vec<Popen>>,
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
}

impl Process {
    pub fn run_thread(option: ProcessOption) -> Sender<Message> {
        let mut process = Self {
            option,
            child: None,
        };
        let (tx, rx) = channel();
        thread::spawn(move || {
            for message in rx {
                match message {
                    Message::Run {
                        env,
                        args,
                        callback,
                    } => {
                        let result = process.run(env, args);
                        callback.send(result).expect("Callback should be success");
                    }
                    Message::Stop {
                        callback,
                    } => {
                        callback.send(Ok(())).expect("Callback should be success");
                    }
                    Message::Quit {
                        callback,
                    } => {
                        callback.send(Ok(())).expect("Callback should be success");
                        break
                    }
                }
            }
        });
        tx
    }

    pub fn run(&mut self, env: String, args: String) -> Result<(), Error> {
        if self.is_running() {
            return Err(Error::AlreadyRunning)
        }

        let args_iter = args.split_whitespace();
        let args_vec: Vec<String> = args_iter.map(|str| str.to_string()).collect();

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
            let kv_array: Vec<&str> = env_kv.split("=").collect();
            if kv_array.len() != 2 {
                return Err(Error::EnvParseError)
            } else {
                ret.push((kv_array[0], kv_array[1]));
            }
        }
        return Ok(ret)
    }
}
