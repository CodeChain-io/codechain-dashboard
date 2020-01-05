use super::super::types::CommitHash;
use super::update::{CallbackResult, Sender};
use super::{git_util, Error};
use crossbeam;
use std::thread;

pub struct Job {}

impl Job {
    pub fn run(codechain_dir: String, commit_hash: CommitHash, callback: crossbeam::Sender<CallbackResult>) -> Sender {
        thread::Builder::new()
            .name("git update job".to_string())
            .spawn(move || {
                let result = Self::update(codechain_dir, &commit_hash);
                callback.send(result);
            })
            .expect("Should success running update job thread")
    }

    fn update(codechain_dir: String, commit_hash: &str) -> Result<(), Error> {
        git_util::remote_update(codechain_dir.clone())?;
        git_util::reset_hard(codechain_dir.clone(), commit_hash.to_string())?;
        let current_hash = git_util::current_hash(codechain_dir)?;
        if commit_hash != current_hash {
            cwarn!(PROCESS, "Updated commit hash not matched expected {} found {}", commit_hash, current_hash);
            Err(Error::Unknown(format!("Cannot update to {}", commit_hash)))
        } else {
            Ok(())
        }
    }
}
