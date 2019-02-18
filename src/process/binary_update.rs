use std::thread;

use crossbeam;
use subprocess::Exec;

use super::update::{CallbackResult, Sender};
use super::Error;

pub struct Job {}

impl Job {
    pub fn run(
        codechain_dir: String,
        binary_url: String,
        binary_checksum: String,
        callback: crossbeam::Sender<CallbackResult>,
    ) -> Sender {
        thread::Builder::new()
            .name("binary update job".to_string())
            .spawn(move || {
                let result = Self::update(codechain_dir, &binary_url, &binary_checksum);
                callback.send(result);
            })
            .expect("Should success running update job thread")
    }

    fn update(codechain_dir: String, binary_url: &str, binary_checksum: &str) -> Result<(), Error> {
        if let Err(err) = move_file(&codechain_dir, "codechain", "codechain.backup") {
            cwarn!(PROCESS, "Cannot move file codechain to codechain.backup: {:?}", err);
        }
        match Self::update_inner(&codechain_dir, binary_url, binary_checksum) {
            Ok(()) => Ok(()),
            Err(err) => {
                if let Err(move_err) = move_file(&codechain_dir, "codechain.backup", "codechain") {
                    cwarn!(PROCESS, "Cannot move file codechain.backup to codechain: {:?}", move_err);
                }
                Err(err)
            }
        }
    }

    fn update_inner(codechain_dir: &str, binary_url: &str, binary_checksum: &str) -> Result<(), Error> {
        download_codechain(&codechain_dir, binary_url)?;
        check_checksum(&codechain_dir, binary_checksum)?;
        make_executable(&codechain_dir)?;
        Ok(())
    }
}

fn move_file(dir: &str, from: &str, to: &str) -> Result<(), Error> {
    cdebug!(PROCESS, "Move {} to {}", from, to);
    let exec = Exec::cmd("mv").arg(from).arg(to).cwd(dir).capture()?;
    if exec.exit_status.success() {
        Ok(())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}

fn download_codechain(codechain_dir: &str, codechain_url: &str) -> Result<(), Error> {
    cdebug!(PROCESS, "RUN wget {}", codechain_url);
    let exec = Exec::cmd("wget").arg(codechain_url).cwd(codechain_dir).capture()?;
    if exec.exit_status.success() {
        Ok(())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}

fn make_executable(codechain_dir: &str) -> Result<(), Error> {
    cdebug!(PROCESS, "Run cmod +x codechain");
    let exec = Exec::cmd("chmod").arg("+x").arg("codechain").cwd(codechain_dir).capture()?;
    if exec.exit_status.success() {
        Ok(())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}

fn check_checksum(codechain_dir: &str, binary_checksum: &str) -> Result<(), Error> {
    cdebug!(PROCESS, "Run shasum codechain | awk '{{ print $1 }}'");
    let shasum = Exec::cmd("shasum").arg("codechain").cwd(codechain_dir);
    let get_1_column = Exec::cmd("awk").arg("{ print $1 }").cwd(codechain_dir);
    let calculated_checksum = { shasum | get_1_column }.capture()?;

    if !calculated_checksum.exit_status.success() {
        return Err(Error::ShellError {
            exit_code: calculated_checksum.exit_status,
            stdout: calculated_checksum.stdout_str(),
            stderr: "".to_string(),
        })
    }

    if calculated_checksum.stdout_str().trim() != binary_checksum.trim() {
        return Err(Error::BinaryChecksumMismatch {
            expected: binary_checksum.trim().to_string(),
            actual: calculated_checksum.stdout_str().trim().to_string(),
        })
    }

    Ok(())
}
