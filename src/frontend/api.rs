use super::super::agent::SendAgentRPC;
use super::super::common_rpc_types::{CommitHash, NodeName, ShellStartCodeChainRequest, ShellUpdateCodeChainRequest};
use super::super::router::Router;
use super::super::rpc::{response, RPCError, RPCResponse};
use super::types::{
    Context, DashboardGetNetworkResponse, DashboardNode, LogGetRequest, LogGetResponse, LogGetTargetsResponse,
    NodeConnection, NodeGetInfoResponse,
};

pub fn add_routing(router: &mut Router<Context>) {
    router.add_route("ping", Box::new(ping as fn(Context) -> RPCResponse<String>));
    router.add_route(
        "node_getInfo",
        Box::new(node_get_info as fn(Context, (String,)) -> RPCResponse<NodeGetInfoResponse>),
    );
    router.add_route(
        "dashboard_getNetwork",
        Box::new(dashboard_get_network as fn(Context) -> RPCResponse<DashboardGetNetworkResponse>),
    );
    router.add_route(
        "node_start",
        Box::new(node_start as fn(Context, (String, ShellStartCodeChainRequest)) -> RPCResponse<()>),
    );
    router.add_route("node_stop", Box::new(node_stop as fn(Context, (String,)) -> RPCResponse<()>));
    router.add_route("node_update", Box::new(node_update as fn(Context, (NodeName, CommitHash)) -> RPCResponse<()>));
    router.add_route(
        "shell_getCodeChainLog",
        Box::new(shell_get_codechain_log as fn(Context, (String,)) -> RPCResponse<String>),
    );
    router.add_route("log_getTargets", Box::new(log_get_targets as fn(Context) -> RPCResponse<LogGetTargetsResponse>));
    router.add_route("log_get", Box::new(log_get as fn(Context, (LogGetRequest,)) -> RPCResponse<LogGetResponse>));
}

fn ping(_: Context) -> RPCResponse<String> {
    response("pong".to_string())
}

fn dashboard_get_network(context: Context) -> RPCResponse<DashboardGetNetworkResponse> {
    let agents_state = context.db_service.get_agents_state()?;
    let connections = context.db_service.get_connections()?;
    let dashboard_nodes = agents_state.iter().map(|agent| DashboardNode::from_db_state(agent)).collect();
    response(DashboardGetNetworkResponse {
        nodes: dashboard_nodes,
        connections: connections.iter().map(|connection| NodeConnection::from_connection(connection)).collect(),
    })
}

fn node_get_info(context: Context, args: (String,)) -> RPCResponse<NodeGetInfoResponse> {
    let (name,) = args;
    let agent_query_result = context.db_service.get_agent_query_result(&name)?.ok_or(RPCError::AgentNotFound)?;
    let extra = context.db_service.get_agent_extra(name)?;
    response(NodeGetInfoResponse::from_db_state(&agent_query_result, &extra))
}

fn node_start(context: Context, args: (NodeName, ShellStartCodeChainRequest)) -> RPCResponse<()> {
    let (name, req) = args;

    let agent = context.agent_service.get_agent(name.clone());
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");
    agent.shell_start_codechain(req.clone())?;

    context.db_service.save_start_option(name, &req.env, &req.args);

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

fn node_update(context: Context, args: (NodeName, CommitHash)) -> RPCResponse<()> {
    let (name, commit_hash) = args;

    let agent = context.agent_service.get_agent(name.clone());
    if agent.is_none() {
        return Err(RPCError::AgentNotFound)
    }
    let agent = agent.expect("Already checked");

    let extra = context.db_service.get_agent_extra(name)?;
    agent.shell_update_codechain(ShellUpdateCodeChainRequest {
        env: extra.as_ref().map(|extra| extra.prev_env.clone()).unwrap_or_default(),
        args: extra.as_ref().map(|extra| extra.prev_args.clone()).unwrap_or_default(),
        commit_hash,
    })?;

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

fn log_get_targets(context: Context) -> RPCResponse<LogGetTargetsResponse> {
    let targets = context.db_service.get_log_targets()?;
    response(LogGetTargetsResponse {
        targets,
    })
}

fn log_get(context: Context, args: (LogGetRequest,)) -> RPCResponse<LogGetResponse> {
    let (req,) = args;
    let logs = context.db_service.get_logs(req)?;
    response(LogGetResponse {
        logs,
    })
}
