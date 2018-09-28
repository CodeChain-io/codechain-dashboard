use std::net::SocketAddr;

use cprimitives::H256;

use super::super::agent;
use super::super::common_rpc_types::{NodeName, NodeStatus};

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
        address: Option<SocketAddr>,
        version: NodeVersion,
        best_block_id: BlockId,
        name: NodeName,
    },
    #[serde(rename_all = "camelCase")]
    UFO {
        status: NodeStatus,
        name: NodeName,
        address: Option<SocketAddr>,
    },
}

impl DashboardNode {
    pub fn from_state(state: &agent::State) -> Option<Self> {
        match state {
            agent::State::Initializing => None,
            agent::State::Normal {
                name,
                status,
                address,
            } => Some(DashboardNode::Normal {
                status: *status,
                name: name.clone(),
                address: *address,
                version: NodeVersion {
                    version: String::new(),
                    hash: String::new(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: H256::new(),
                },
            }),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnection {
    pub node_a: String,
    pub node_b: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardGetNetworkResponse {
    pub nodes: Vec<DashboardNode>,
    pub connections: Vec<NodeConnection>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOption {
    pub env: String,
    pub args: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGetInfoResponse {
    pub name: NodeName,
    pub status: NodeStatus,
    pub start_option: Option<StartOption>,
    pub address: Option<SocketAddr>,
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
            name: "Dummy".to_string(),
            address: Some("127.0.0.1:3485".parse().unwrap()),
            version: NodeVersion {
                version: "0.1.0".to_string(),
                hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
            },
            status: NodeStatus::Run,
            start_option: None,
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

    pub fn from_state(state: &agent::State) -> Option<Self> {
        match state {
            agent::State::Initializing => None,
            agent::State::Normal {
                status,
                address,
                name,
            } => {
                let mut dummy = Self::dummy();
                dummy.address = *address;
                dummy.status = *status;
                dummy.name = name.clone();
                Some(dummy)
            }
        }
    }
}
