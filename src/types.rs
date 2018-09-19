use std::net::SocketAddr;
use std::sync::mpsc::Sender;

use super::process::Message as ProcessMessage;

pub struct AgentArgs<'a> {
    pub codechain_dir: &'a str,
    pub log_file_path: &'a str,
    pub hub_url: &'a str,
    pub codechain_address: SocketAddr,
}

pub struct HandlerContext {
    pub process: Sender<ProcessMessage>,
    pub codechain_address: SocketAddr,
}
