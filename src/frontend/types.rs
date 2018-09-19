use std::net::SocketAddr;

use cprimitives::H256;

use super::super::agent;
use super::super::agent::agent::State as AgentState;
use super::super::common_rpc_types::NodeStatus;

#[derive(Clone)]
pub struct Context {
    pub agent_service: agent::ServiceSender,
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
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
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

impl DashboardNode {
    pub fn from_state(state: &AgentState) -> Option<Self> {
        match state {
            AgentState::Initializing => None,
            AgentState::Normal {
                status,
                address,
            } => Some(DashboardNode::Normal {
                status: *status,
                address: *address,
                version: NodeVersion {
                    version: String::new(),
                    hash: String::new(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: H256::new(),
                },
                name: None,
            }),
        }
    }
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
    pub status: NodeStatus,
    pub version: NodeVersion,
    pub best_block_id: BlockId,
    pub pending_parcels: Vec<Parcel>,
    pub peers: Vec<SocketAddr>,
    pub whitelist: NetworkPermission,
    pub blacklist: NetworkPermission,
    pub hardware: HardwareInfo,
    pub events: Vec<Event>,
}

impl NodeGetInfoResponse {
    fn dummy() -> Self {
        NodeGetInfoResponse {
            address: "127.0.0.1:3485".parse().unwrap(),
            version: NodeVersion {
                version: "0.1.0".to_string(),
                hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
            },
            status: NodeStatus::Run,
            best_block_id: BlockId {
                block_number: 0,
                hash: Default::default(),
            },
            pending_parcels: Vec::new(),
            peers: Vec::new(),
            whitelist: NetworkPermission {
                list: Vec::new(),
                enabled: false,
            },
            blacklist: NetworkPermission {
                list: Vec::new(),
                enabled: false,
            },
            hardware: HardwareInfo {
                cpu_usage: vec![0.34, 0.03, 0.58],
                disk_usage: HardwareUsage {
                    total: 8 * 1000 * 1000 * 1000,
                    available: 5 * 1000 * 1000 * 1000,
                    percentage_used: 0.6,
                },
                memory_usage: HardwareUsage {
                    total: 8 * 1000 * 1000 * 1000,
                    available: 5 * 1000 * 1000 * 1000,
                    percentage_used: 0.6,
                },
            },
            events: vec!["Network connected".to_string(), "Block received".to_string()],
        }
    }

    pub fn from_state(state: &AgentState) -> Option<Self> {
        match state {
            AgentState::Initializing => None,
            AgentState::Normal {
                status,
                address,
            } => {
                let mut dummy = Self::dummy();
                dummy.address = *address;
                dummy.status = *status;
                Some(dummy)
            }
        }
    }
}
