use super::process::Message as ProcessMessage;
use std::sync::mpsc::Sender;

pub struct HandlerContext {
    pub process: Sender<ProcessMessage>,
}
