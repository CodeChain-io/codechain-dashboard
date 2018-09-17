use super::router::Router;
use super::types::{
    BlockId, DashboardGetNetworkResponse, DashboardNode, HardwareInfo, HardwareUsage, NetworkPermission,
    NodeConnection, NodeGetInfoResponse, NodeStatus, NodeVersion,
};

pub fn add_routing(router: &mut Router) {
    let f: fn() -> String = ping;
    router.add_route("ping", Box::new(f));
    router.add_route("ping", Box::new(ping as fn() -> String));
    router.add_route("add1", Box::new(add1 as fn(i32) -> i32));
    router.add_route("dashboard_getNetwork", Box::new(dashboard_get_network as fn() -> DashboardGetNetworkResponse));
    router.add_route("node_getInfo", Box::new(node_get_info as fn() -> NodeGetInfoResponse));
}

fn ping() -> String {
    "pong".to_string()
}

fn add1(x: i32) -> i32 {
    x + 1
}

fn dashboard_get_network() -> DashboardGetNetworkResponse {
    DashboardGetNetworkResponse {
        nodes: vec![
            DashboardNode::Normal {
                name: Some("Gilyoung".to_string()),
                status: NodeStatus::Run,
                address: "127.0.0.1:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: None,
                status: NodeStatus::Run,
                address: "127.0.0.2:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: Some("Hi stopped test node1".to_string()),
                status: NodeStatus::Stop,
                address: "42.124.241.2:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::Normal {
                name: Some("Test Error node".to_string()),
                status: NodeStatus::Error,
                address: "127.0.0.3:3485".parse().unwrap(),
                version: NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                },
                best_block_id: BlockId {
                    block_number: 0,
                    hash: Default::default(),
                },
            },
            DashboardNode::UFO {
                status: NodeStatus::UFO,
                address: "2.2.2.2:3485".parse().unwrap(),
            },
        ],
        connections: vec![NodeConnection {
            node_a: "127.0.0.1:3485".parse().unwrap(),
            node_b: "127.0.0.2:3485".parse().unwrap(),
        }],
    }
}

fn node_get_info() -> NodeGetInfoResponse {
    NodeGetInfoResponse {
        address: "127.0.0.1:3485".parse().unwrap(),
        version: NodeVersion {
            version: "0.1.0".to_string(),
            hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
        },
        commit_hash: "84e70586dea8e6b4021d65b8164bbac28cb88ecb".to_string(),
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
            cpu_usage: 3.4,
            disk_usage: HardwareUsage {
                total: "3GB".to_string(),
                available: "5GB".to_string(),
                percentage_used: "60%".to_string(),
            },
            memory_usage: HardwareUsage {
                total: "3GB".to_string(),
                available: "5GB".to_string(),
                percentage_used: "60%".to_string(),
            },
        },
        events: vec!["Network connected".to_string(), "Block received".to_string()],
    }
}
