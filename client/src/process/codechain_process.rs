extern crate libc;
extern crate reopen;

use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::Mutex;
use subprocess::{Exec, ExitStatus, Popen, PopenError, Redirection};

use super::ProcessOption;

use self::reopen::Reopen;

#[derive(Clone)]
pub struct CodeChainProcess {
    process: Arc<Mutex<Popen>>,
}

impl CodeChainProcess {
    pub fn new(envs: Vec<(&str, &str)>, args: Vec<String>, option: &ProcessOption) -> Result<Self, String> {
        let log_file_path = option.log_file_path.clone();
        let mut file =
            Reopen::new(Box::new(move || OpenOptions::new().append(true).create(true).open(log_file_path.clone())))
                .map_err(|err| err.to_string())?;
        file.handle().register_signal(libc::SIGHUP).unwrap();

        let mut exec = if Path::new(&option.codechain_dir).join("codechain").exists() {
            Exec::cmd("./codechain")
                .cwd(option.codechain_dir.clone())
                .stdout(Redirection::Pipe)
                .stderr(Redirection::Merge)
                .args(&args)
        } else {
            Exec::cmd("cargo")
                .arg("run")
                .arg("--")
                .cwd(option.codechain_dir.clone())
                .stdout(Redirection::Pipe)
                .stderr(Redirection::Merge)
                .args(&args)
        };

        for (k, v) in envs {
            exec = exec.env(k, v);
        }

        let child = exec.popen().map_err(|err| err.to_string())?;

        let process = CodeChainProcess {
            process: Arc::new(Mutex::new(child)),
        };

        let process_in_thread = process.clone();

        thread::Builder::new()
            .name("codechain_log_writer".to_string())
            .spawn(move || {
                let mut buf: [u8; 1024] = [0; 1024];
                loop {
                    let length = match process_in_thread.read(&mut buf) {
                        Ok(length) => length,
                        Err(err) => {
                            cerror!(PROCESS, "Fail to read stdout of CodeChain : {}", err);
                            return
                        }
                    };

                    if let Err(err) = file.write_all(&buf[0..length]) {
                        cerror!(PROCESS, "Fail to write stdout of CodeChain : {}", err);
                        return
                    }
                }
            })
            .expect("Should success running process thread");

        Ok(process)
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut process = self.process.lock();
        process.stdout.as_mut().expect("Process opened with pipe").read(buf)
    }

    pub fn is_running(&self) -> bool {
        let mut process = self.process.lock();
        process.poll().is_none()
    }

    pub fn terminate(&self) -> Result<(), io::Error> {
        let mut process = self.process.lock();
        process.terminate()
    }

    pub fn wait_timeout(&self, duration: Duration) -> Result<Option<ExitStatus>, PopenError> {
        let mut process = self.process.lock();
        process.wait_timeout(duration)
    }

    pub fn kill(&self) -> Result<(), io::Error> {
        let mut process = self.process.lock();
        process.kill()
    }
}
