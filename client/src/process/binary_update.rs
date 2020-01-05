use super::update::{CallbackResult, Sender};
use super::{fs_util, Error};
use crossbeam;
use std::thread;

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
        if let Err(err) = fs_util::move_file(&codechain_dir, "codechain", "codechain.backup") {
            cwarn!(PROCESS, "Cannot move file codechain to codechain.backup: {:?}", err);
        }
        match Self::update_inner(&codechain_dir, binary_url, binary_checksum) {
            Ok(()) => Ok(()),
            Err(err) => {
                if let Err(move_err) = fs_util::move_file(&codechain_dir, "codechain.backup", "codechain") {
                    cwarn!(PROCESS, "Cannot move file codechain.backup to codechain: {:?}", move_err);
                }
                Err(err)
            }
        }
    }

    fn update_inner(codechain_dir: &str, binary_url: &str, binary_checksum: &str) -> Result<(), Error> {
        fs_util::download_codechain(&codechain_dir, binary_url)?;
        fs_util::check_checksum(&codechain_dir, binary_checksum)?;
        fs_util::make_executable(&codechain_dir)?;
        Ok(())
    }
}
