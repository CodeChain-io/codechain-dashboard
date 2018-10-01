use super::super::agent::SendAgentRPC;
use super::super::common_rpc_types::{
    BlackList, BlockId, NodeName, NodeStatus, NodeVersion, ShellStartCodeChainRequest, WhiteList,
};
use super::super::router::Router;
use super::super::rpc::{response, RPCError, RPCResponse};
use super::types::{
    Context, DashboardGetNetworkResponse, DashboardNode, HardwareInfo, HardwareUsage, NodeConnection,
    NodeGetInfoResponse, StartOption,
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
        Box::new(real_node_get_info as fn(Context, (String,)) -> RPCResponse<NodeGetInfoResponse>),
    );
    router.add_route(
        "node_start",
        Box::new(node_start as fn(Context, (String, ShellStartCodeChainRequest)) -> RPCResponse<()>),
    );
    router.add_route("node_stop", Box::new(node_stop as fn(Context, (String,)) -> RPCResponse<()>));
    router.add_route(
        "shell_getCodeChainLog",
        Box::new(shell_get_codechain_log as fn(Context, (String,)) -> RPCResponse<String>),
    );
}

fn ping(_: Context) -> RPCResponse<String> {
    response("pong".to_string())
}

fn dashboard_get_network(_: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    response(DashboardGetNetworkResponse {
        nodes: vec![
            DashboardNode::Normal {
                name: "Gilyoung".to_string(),
                status: NodeStatus::Run,
                address: Some("127.0.0.1:3485".parse().unwrap()),
                version: Some(NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                }),
                best_block_id: Some(BlockId {
                    block_number: 0,
                    hash: Default::default(),
                }),
            },
            DashboardNode::Normal {
                name: "Juhyung".to_string(),
                status: NodeStatus::Run,
                address: Some("127.0.0.2:3486".parse().unwrap()),
                version: Some(NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                }),
                best_block_id: Some(BlockId {
                    block_number: 0,
                    hash: Default::default(),
                }),
            },
            DashboardNode::Normal {
                name: "Hi stopped test node1".to_string(),
                status: NodeStatus::Stop,
                address: Some("42.124.241.2:3487".parse().unwrap()),
                version: Some(NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                }),
                best_block_id: Some(BlockId {
                    block_number: 0,
                    hash: Default::default(),
                }),
            },
            DashboardNode::Normal {
                name: "Starting node".to_string(),
                status: NodeStatus::Starting,
                address: Some("127.0.0.3:3488".parse().unwrap()),
                version: Some(NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                }),
                best_block_id: Some(BlockId {
                    block_number: 0,
                    hash: Default::default(),
                }),
            },
            DashboardNode::Normal {
                name: "Test Error node".to_string(),
                status: NodeStatus::Error,
                address: Some("127.0.0.3:3489".parse().unwrap()),
                version: Some(NodeVersion {
                    version: "0.1.0".to_string(),
                    hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
                }),
                best_block_id: Some(BlockId {
                    block_number: 0,
                    hash: Default::default(),
                }),
            },
            DashboardNode::UFO {
                status: NodeStatus::UFO,
                name: "UTF-1".to_string(),
                address: Some("2.2.2.2:3410".parse().unwrap()),
            },
        ],
        connections: vec![NodeConnection {
            node_a: "Gilyoung".to_string(),
            node_b: "Juhyung".to_string(),
        }],
    })
}

fn real_dashboard_get_network(context: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    let agents_state = context.db_service.get_agents_state();
    let connections = context.db_service.get_connections();
    let dashboard_nodes = agents_state.iter().map(|agent| DashboardNode::from_db_state(agent)).collect();
    response(DashboardGetNetworkResponse {
        nodes: dashboard_nodes,
        connections: connections.iter().map(|connection| NodeConnection::from_connection(connection)).collect(),
    })
}

fn real_node_get_info(context: Context, args: (String,)) -> RPCResponse<NodeGetInfoResponse> {
    let (name,) = args;
    let agent_query_result = context.db_service.get_agent_query_result(&name).ok_or(RPCError::AgentNotFound)?;
    let extra = context.db_service.get_agent_extra(&name);
    response(NodeGetInfoResponse::from_db_state(&agent_query_result, &extra))
}

fn node_get_info(_: Context) -> RPCResponse<NodeGetInfoResponse> {
    response(NodeGetInfoResponse {
        address: Some("127.0.0.1:3485".parse().unwrap()),
        name: "Dummy".to_string(),
        version: Some(NodeVersion {
            version: "0.1.0".to_string(),
            hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
        }),
        status: NodeStatus::Stop,
        start_option: Some(StartOption {
            env: "RUST_LOG=trace".to_string(),
            args: "-c husky".to_string(),
        }),
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

fn node_start(context: Context, args: (NodeName, ShellStartCodeChainRequest)) -> RPCResponse<()> {
    let (name, req) = args;

    let agent = context.agent_service.get_agent(name.clone());
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    agent.shell_start_codechain(req.clone())?;

    context.db_service.save_start_option(&name, &req.env, &req.args);

    response(())
}

fn node_stop(context: Context, args: (String,)) -> RPCResponse<()> {
    let (name,) = args;

    let agent = context.agent_service.get_agent(name);
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    agent.shell_stop_codechain()?;

    response(())
}

fn shell_get_codechain_log(context: Context, args: (String,)) -> RPCResponse<String> {
    let (name,) = args;

    let agent = context.agent_service.get_agent(name);
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    let result = agent.shell_get_codechain_log()?;

    response(result)
}
