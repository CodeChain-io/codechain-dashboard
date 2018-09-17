use std::net::SocketAddr;

use cprimitives::H256;
use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

pub type RPCResult<T> = Result<Option<T>, RPCError>;

pub enum RPCError {
    Internal(String),
}

pub fn response<T>(value: T) -> RPCResult<T> {
    Ok(Some(value))
}

impl RPCError {
    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        match self {
            RPCError::Internal(str) => Self::create_internal_rpc_error(str),
        }
    }

    fn create_internal_rpc_error(msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::InternalError);
        ret.data = Some(Value::String(msg.to_string()));
        ret
    }
}

impl From<SerdeError> for RPCError {
    fn from(err: SerdeError) -> Self {
        RPCError::Internal(format!("Internal error about JSON serialize/deserialize : {:?}", err))
    }
}

pub type Event = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
    // FIXME: fill structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPermission {
    pub list: Vec<SocketAddr>,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockId {
    pub block_number: i64,
    pub hash: H256,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeVersion {
    pub version: String,
    pub hash: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: String,
    pub available: String,
    pub percentage_used: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: f64,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

#[derive(Debug, Serialize)]
pub enum NodeStatus {
    Run,
    Stop,
    Error,
    UFO,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DashboardNode {
    #[serde(rename_all = "camelCase")]
    Normal {
        status: NodeStatus,
        address: SocketAddr,
        version: NodeVersion,
        best_block_id: BlockId,
        name: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    UFO {
        status: NodeStatus,
        address: SocketAddr,
    },
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGetInfoResponse {
    pub address: SocketAddr,
    pub version: NodeVersion,
    pub commit_hash: String,
    pub best_block_id: BlockId,
    pub pending_parcels: Vec<Parcel>,
    pub peers: Vec<SocketAddr>,
    pub whitelist: NetworkPermission,
    pub blacklist: NetworkPermission,
    pub hardware: HardwareInfo,
    pub events: Vec<Event>,
}
