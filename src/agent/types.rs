use std::net::SocketAddr;

use serde_json::Value;

use super::super::common_rpc_types::NodeStatus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGetInfoResponse {
    pub status: NodeStatus,
    pub name: String,
    pub address: Option<SocketAddr>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeChainCallRPCResponse {
    pub inner_response: Value,
}
