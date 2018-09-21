use std::net::SocketAddr;

use super::super::agent::agent::SendAgentRPC;
use super::super::common_rpc_types::{NodeStatus, ShellStartCodeChainRequest};
use super::super::router::Router;
use super::super::rpc::{response, RPCError, RPCResponse};
use super::types::{
    BlockId, Context, DashboardGetNetworkResponse, DashboardNode, HardwareInfo, HardwareUsage, NetworkPermission,
    NodeConnection, NodeGetInfoResponse, NodeVersion, StartOption,
};

pub fn add_routing(router: &mut Router<Context>) {
    router.add_route("ping", Box::new(ping as fn(Context) -> RPCResponse<String>));
    router.add_route(
        "dashboard_getNetwork",
        Box::new(dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route("node_getInfo", Box::new(node_get_info as fn(Context) -> RPCResponse<NodeGetInfoResponse>));
    router.add_route(
        "real_dashboard_getNetwork",
        Box::new(real_dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route(
        "real_node_getInfo",
        Box::new(real_node_get_info as fn(Context, (SocketAddr,)) -> RPCResponse<NodeGetInfoResponse>),
    );
    router.add_route(
        "node_start",
        Box::new(node_start as fn(Context, (SocketAddr, ShellStartCodeChainRequest)) -> RPCResponse<()>),
    );
    router.add_route("node_stop", Box::new(node_stop as fn(Context, (SocketAddr,)) -> RPCResponse<()>));
    router.add_route(
        "shell_getCodeChainLog",
        Box::new(shell_get_codechain_log as fn(Context, (SocketAddr,)) -> RPCResponse<String>),
    );
}

fn ping(_: Context) -> RPCResponse<String> {
    response("pong".to_string())
}

fn dashboard_get_network(_: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    response(DashboardGetNetworkResponse {
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
                address: "127.0.0.2:3486".parse().unwrap(),
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
                address: "42.124.241.2:3487".parse().unwrap(),
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
                name: Some("Starting node".to_string()),
                status: NodeStatus::Starting,
                address: "127.0.0.3:3488".parse().unwrap(),
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
                address: "127.0.0.3:3489".parse().unwrap(),
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
                address: "2.2.2.2:3410".parse().unwrap(),
            },
        ],
        connections: vec![NodeConnection {
            node_a: "127.0.0.1:3485".parse().unwrap(),
            node_b: "127.0.0.2:3486".parse().unwrap(),
        }],
    })
}

fn real_dashboard_get_network(context: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    let agent_infos = context.agent_service.read_state().get_agent_info();
    let dashboard_nodes = agent_infos.iter().filter_map(DashboardNode::from_state).collect();
    response(DashboardGetNetworkResponse {
        nodes: dashboard_nodes,
        connections: Vec::new(),
    })
}

fn real_node_get_info(context: Context, args: (SocketAddr,)) -> RPCResponse<NodeGetInfoResponse> {
    let (address,) = args;
    let agent = context.agent_service.get_agent(address).ok_or(RPCError::AgentNotFound)?;
    let state = agent.read_state();
    let node_response = NodeGetInfoResponse::from_state(&*state).ok_or(RPCError::AgentNotFound)?;
    response(node_response)
}

fn node_get_info(_: Context) -> RPCResponse<NodeGetInfoResponse> {
    response(NodeGetInfoResponse {
        address: "127.0.0.1:3485".parse().unwrap(),
        version: NodeVersion {
            version: "0.1.0".to_string(),
            hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
        },
        status: NodeStatus::Stop,
        start_option: Some(StartOption {
            env: "RUST_LOG=trace".to_string(),
            args: "-c husky".to_string(),
        }),
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
            cpu_usage: vec![0.95, 0.03, 0.58],
            disk_usage: HardwareUsage {
                total: 512 * 1000 * 1000 * 1000,
                available: 10 * 1000 * 1000 * 1000,
                percentage_used: 0.6,
            },
            memory_usage: HardwareUsage {
                total: 8 * 1000 * 1000 * 1000,
                available: 4 * 1000 * 1000 * 100,
                percentage_used: 0.6,
            },
        },
        events: vec!["Network connected".to_string(), "Block received".to_string()],
    })
}

fn node_start(context: Context, args: (SocketAddr, ShellStartCodeChainRequest)) -> RPCResponse<()> {
    let (address, req) = args;

    let agent = context.agent_service.get_agent(address);
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    agent.shell_start_codechain(req)?;

    response(())
}

fn node_stop(context: Context, args: (SocketAddr,)) -> RPCResponse<()> {
    let (address,) = args;

    let agent = context.agent_service.get_agent(address);
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    agent.shell_stop_codechain()?;

    response(())
}

fn shell_get_codechain_log(context: Context, args: (SocketAddr,)) -> RPCResponse<String> {
    let (address,) = args;

    let agent = context.agent_service.get_agent(address);
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    let result = agent.shell_get_codechain_log()?;

    response(result)
}
