use std::net::SocketAddr;

use super::super::common_rpc_types::{NodeName, NodeStatus};

#[derive(PartialEq, Clone, Debug)]
pub struct AgentState {
    pub name: NodeName,
    pub status: NodeStatus,
    pub address: Option<SocketAddr>,
}
