use std::path::Path;

use subprocess::Exec;

use super::Error;

pub fn move_file(dir: &str, from: &str, to: &str) -> Result<(), Error> {
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

pub fn download_codechain(codechain_dir: &str, codechain_url: &str) -> Result<(), Error> {
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

pub fn make_executable(codechain_dir: &str) -> Result<(), Error> {
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

pub fn check_checksum(codechain_dir: &str, binary_checksum: &str) -> Result<(), Error> {
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

pub fn get_checksum_or_default(dir: &str, file: &str) -> Result<String, Error> {
    let path = Path::new(dir).join(file);
    if !path.exists() {
        return Ok("".to_string())
    }

    cdebug!(PROCESS, "Run shasum {:?}", path);
    let exec = Exec::cmd("shasum").arg("-a").arg("256").arg(file).cwd(dir).capture()?;

    if exec.exit_status.success() {
        Ok(exec.stdout_str().trim().to_string())
    } else {
        Err(Error::ShellError {
            exit_code: exec.exit_status,
            stdout: exec.stdout_str(),
            stderr: exec.stderr_str(),
        })
    }
}
