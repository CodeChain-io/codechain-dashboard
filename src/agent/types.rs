use std::net::SocketAddr;

use super::super::common_rpc_types::NodeStatus;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGetInfoResponse {
    pub status: NodeStatus,
    pub address: SocketAddr,
}
