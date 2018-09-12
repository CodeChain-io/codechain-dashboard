use super::router::Router;
use super::types::{BlockId, DashboardGetNetworkResponse, DashboardNode};

pub fn add_routing(routing_table: &mut Router) {
    let f: fn() -> String = ping;
    routing_table.add_route("ping", Box::new(f));
    routing_table.add_route("ping", Box::new(ping as fn() -> String));
    routing_table.add_route("add1", Box::new(add1 as fn(i32) -> i32));
    routing_table
        .add_route("dashboard_getNetwork", Box::new(dashboard_get_network as fn() -> DashboardGetNetworkResponse))
}

fn ping() -> String {
    "pong".to_string()
}

fn add1(x: i32) -> i32 {
    x + 1
}

fn dashboard_get_network() -> DashboardGetNetworkResponse {
    DashboardGetNetworkResponse {
        nodes: vec![DashboardNode {
            address: "127.0.0.1:3485".parse().unwrap(),
            version: "0.1.0".to_string(),
            best_block_id: BlockId {
                number: 0,
                hash: Default::default(),
            },
            pending_parcel_count: 0,
        }],
        connections: Vec::new(),
    }
}
