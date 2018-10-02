use std::net::IpAddr;
use std::sync::mpsc::Sender;

use super::hardware_usage::HardwareService;
use super::process::Message as ProcessMessage;

pub struct AgentArgs<'a> {
    pub codechain_dir: &'a str,
    pub log_file_path: &'a str,
    pub hub_url: &'a str,
    pub codechain_address: IpAddr,
    pub name: &'a str,
}

pub struct HandlerContext {
    pub process: Sender<ProcessMessage>,
    pub codechain_address: IpAddr,
    pub name: String,
    pub hardware_service: HardwareService,
}
