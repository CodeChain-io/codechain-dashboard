use super::Error;
use std::thread::JoinHandle;

pub type Sender = JoinHandle<()>;
pub type CallbackResult = Result<(), Error>;
