use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

use super::Error;

use super::super::types::CommitHash;
use super::git_util;

pub struct Job {}

pub type Sender = JoinHandle<()>;
pub type CallbackResult = Result<(), Error>;

impl Job {
    pub fn run(codechain_dir: String, commit_hash: CommitHash, callback: mpsc::Sender<CallbackResult>) -> Sender {
        thread::Builder::new()
            .name("update job".to_string())
            .spawn(move || {
                let result = Self::update(codechain_dir, commit_hash);
                if let Err(err) = callback.send(result) {
                    cerror!(PROCESS, "Cannot run callback from git_update : {}", err);
                }
            })
            .expect("Should success running update job thread")
    }

    fn update(codechain_dir: String, commit_hash: CommitHash) -> Result<(), Error> {
        git_util::remote_update(codechain_dir.clone())?;
        git_util::reset_hard(codechain_dir.clone(), commit_hash.to_string())?;
        let current_hash = git_util::current_hash(codechain_dir.clone())?;
        if commit_hash != current_hash {
            cwarn!(PROCESS, "Updated commit hash not matched expected {} found {}", commit_hash, current_hash);
            Err(Error::Unknown(format!("Cannot update to {}", commit_hash)))
        } else {
            Ok(())
        }
    }
}
