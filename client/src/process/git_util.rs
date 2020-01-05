use super::super::types::CommitHash;
use super::{Error, Exec};

pub fn current_hash(codechain_dir: String) -> Result<CommitHash, Error> {
    cdebug!(PROCESS, "Run git rev-parse HEAD at {}", codechain_dir);
    let result = match Exec::cmd("git").arg("rev-parse").arg("HEAD").cwd(codechain_dir).capture() {
        Ok(exec) => exec.stdout_str().trim().to_string(),
        Err(err) => {
            cwarn!(PROCESS, "Cannot get git hash {}", err);
            "NONE".to_string()
        }
    };
    Ok(result)
}

pub fn remote_update(codechain_dir: String) -> Result<(), Error> {
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

pub fn reset_hard(codechain_dir: String, target_commit_hash: CommitHash) -> Result<(), Error> {
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
