use std::net::SocketAddr;

use super::super::agent;
use super::super::common_rpc_types;
use super::super::common_rpc_types::{
    BlackList, BlockId, HardwareInfo, HardwareUsage, NodeName, NodeStatus, NodeVersion, PendingParcel, WhiteList,
};
use super::super::db;

#[derive(Clone)]
pub struct Context {
    pub agent_service: agent::ServiceSender,
    pub db_service: db::ServiceSender,
}

pub type Event = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DashboardNode {
    #[serde(rename_all = "camelCase")]
    Normal {
        status: NodeStatus,
        address: Option<SocketAddr>,
        version: Option<NodeVersion>,
        best_block_id: Option<BlockId>,
        name: NodeName,
    },
    #[serde(rename_all = "camelCase")]
    #[allow(dead_code)]
    UFO {
        status: NodeStatus,
        name: NodeName,
        address: Option<SocketAddr>,
    },
}

impl DashboardNode {
    pub fn from_db_state(state: &db::AgentQueryResult) -> Self {
        DashboardNode::Normal {
            status: state.status,
            name: state.name.clone(),
            address: state.address,
            version: state.version.clone(),
            best_block_id: state.best_block_id,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnection {
    pub node_a: String,
    pub node_b: String,
}

impl NodeConnection {
    pub fn from_connection(connection: &common_rpc_types::Connection) -> Self {
        let (node_a, node_b) = connection;
        Self {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
        }
    }
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
    pub version: Option<NodeVersion>,
    pub best_block_id: Option<BlockId>,
    pub pending_parcels: Vec<PendingParcel>,
    pub peers: Vec<SocketAddr>,
    pub whitelist: Option<WhiteList>,
    pub blacklist: Option<BlackList>,
    pub hardware: Option<HardwareInfo>,
    pub events: Vec<Event>,
}

impl NodeGetInfoResponse {
    fn dummy() -> Self {
        NodeGetInfoResponse {
            name: "Dummy".to_string(),
            address: Some("127.0.0.1:3485".parse().unwrap()),
            version: Some(NodeVersion {
                version: "0.1.0".to_string(),
                hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
            }),
            status: NodeStatus::Run,
            start_option: None,
            best_block_id: Some(BlockId {
                block_number: 0,
                hash: Default::default(),
            }),
            pending_parcels: Vec::new(),
            peers: Vec::new(),
            whitelist: Some(WhiteList {
                list: Vec::new(),
                enabled: false,
            }),
            blacklist: Some(BlackList {
                list: Vec::new(),
                enabled: false,
            }),
            hardware: Some(HardwareInfo {
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
            }),
            events: vec!["Network connected".to_string(), "Block received".to_string()],
        }
    }

    pub fn from_db_state(state: &db::AgentQueryResult, extra: &Option<db::AgentExtra>) -> Self {
        let mut dummy = Self::dummy();
        dummy.address = state.address;
        dummy.status = state.status;
        dummy.name = state.name.clone();
        dummy.peers = state.peers.clone();
        dummy.best_block_id = state.best_block_id;
        dummy.version = state.version.clone();
        dummy.pending_parcels = state.pending_parcels.clone();
        dummy.whitelist = state.whitelist.clone();
        dummy.blacklist = state.blacklist.clone();
        dummy.start_option = extra.as_ref().map(|extra| StartOption {
            env: extra.prev_env.clone(),
            args: extra.prev_args.clone(),
        });
        dummy.hardware = state.hardware.clone();
        dummy
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogGetTargetsResponse {
    pub targets: Vec<String>,
}

pub type LogGetRequest = db::LogQueryParams;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogGetResponse {
    pub logs: Vec<db::Log>,
}
