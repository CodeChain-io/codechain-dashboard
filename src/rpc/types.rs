use std::net::SocketAddr;

use cprimitives::H256;

pub type NodeVersion = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockId {
    pub number: i64,
    pub hash: H256,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardNode {
    pub address: SocketAddr,
    pub version: NodeVersion,
    pub best_block_id: BlockId,
    pub pending_parcel_count: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnection {
    pub node_a: SocketAddr,
    pub node_b: SocketAddr,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardGetNetworkResponse {
    pub nodes: Vec<DashboardNode>,
    pub connections: Vec<NodeConnection>,
}
