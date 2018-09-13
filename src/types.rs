use super::process::Message as ProcessMessage;
use std::sync::mpsc::Sender;

pub struct AgentArgs<'a> {
    pub codechain_dir: &'a str,
    pub log_file_path: &'a str,
    pub hub_url: &'a str,
}

pub struct HandlerContext {
    pub process: Sender<ProcessMessage>,
}
