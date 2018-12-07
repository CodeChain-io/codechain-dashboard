use std::net::SocketAddr;

use crossbeam::channel;
use serde_json::Value;

use super::super::hardware_usage::HardwareInfo;
use super::super::process::{Error as ProcessError, Message as ProcessMessage};
use super::super::types::HandlerContext;
use super::router::Router;
use super::types::{
    response, AgentGetInfoResponse, CodeChainCallRPCResponse, RPCResult, ShellStartCodeChainRequest,
    ShellUpdateCodeChainRequest,
};
use rpc::types::RPCError;
use rpc::types::ERR_NETWORK_ERROR;

pub fn add_routing(router: &mut Router) {
    router.add_route("ping", Box::new(ping as fn(&HandlerContext) -> RPCResult<String>));
    router.add_route(
        "shell_startCodeChain",
        Box::new(shell_start_codechain as fn(&HandlerContext, (ShellStartCodeChainRequest,)) -> RPCResult<()>),
    );
    router.add_route("shell_stopCodeChain", Box::new(shell_stop_codechain as fn(&HandlerContext) -> RPCResult<()>));
    router.add_route(
        "shell_updateCodeChain",
        Box::new(shell_update_codechain as fn(&HandlerContext, (ShellUpdateCodeChainRequest,)) -> RPCResult<()>),
    );
    router.add_route(
        "shell_getCodeChainLog",
        Box::new(shell_get_codechain_log as fn(&HandlerContext) -> RPCResult<String>),
    );
    router
        .add_route("agent_getInfo", Box::new(agent_get_info as fn(&HandlerContext) -> RPCResult<AgentGetInfoResponse>));
    router.add_route(
        "codechain_callRPC",
        Box::new(
            codechain_call_rpc as fn(&HandlerContext, (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse>,
        ),
    );
    router.add_route("hardware_get", Box::new(hardware_get as fn(&HandlerContext) -> RPCResult<HardwareInfo>));
}

fn ping(_context: &HandlerContext) -> RPCResult<String> {
    response("pong".to_string())
}

fn shell_start_codechain(context: &HandlerContext, req: (ShellStartCodeChainRequest,)) -> RPCResult<()> {
    let (req,) = req;

    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::Run {
        env: req.env,
        args: req.args,
        callback: tx,
    });
    let process_result = rx.recv();
    process_result.ok_or_else(|| RPCError::Internal("Cannot receive  process result".to_string()))??;
    response(())
}

fn shell_stop_codechain(context: &HandlerContext) -> RPCResult<()> {
    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::Stop {
        callback: tx,
    });
    let process_result = rx.recv();
    process_result.ok_or_else(|| RPCError::Internal("Cannot receive  process result".to_string()))??;
    response(())
}

fn shell_update_codechain(context: &HandlerContext, req: (ShellUpdateCodeChainRequest,)) -> RPCResult<()> {
    let (req,) = req;

    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::Update {
        env: req.env,
        args: req.args,
        target_version: req.commit_hash,
        callback: tx,
    });
    let process_result = rx.recv();
    process_result.ok_or_else(|| RPCError::Internal("Cannot receive  process result".to_string()))??;
    response(())
}

fn shell_get_codechain_log(context: &HandlerContext) -> RPCResult<String> {
    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::GetLog {
        callback: tx,
    });
    let process_result = rx.recv();
    let result = process_result.ok_or_else(|| RPCError::Internal("Cannot receive  process result".to_string()))??;
    response(result)
}

fn agent_get_info(context: &HandlerContext) -> RPCResult<AgentGetInfoResponse> {
    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::GetStatus {
        callback: tx,
    });
    let process_result = rx.recv().ok_or_else(|| RPCError::Internal("Cannot get process result".to_string()))?;
    let (node_status, port, commit_hash) = process_result?;
    response(AgentGetInfoResponse {
        name: context.name.clone(),
        status: node_status,
        address: port.map(|port| SocketAddr::new(context.codechain_address, port)),
        codechain_commit_hash: commit_hash,
    })
}

fn codechain_call_rpc(context: &HandlerContext, args: (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse> {
    let (method, arguments) = args;
    let (tx, rx) = channel::unbounded();
    context.process.send(ProcessMessage::CallRPC {
        method,
        arguments,
        callback: tx,
    });
    let process_result = rx.recv().ok_or_else(|| RPCError::Internal("Cannot receive  process result".to_string()))?;

    let value = match process_result {
        Ok(value) => value,
        Err(ProcessError::CodeChainRPC(_)) => {
            return Err(RPCError::ErrorResponse(ERR_NETWORK_ERROR, "Network Error".to_string(), None))
        }
        Err(err) => return Err(err.into()),
    };
    response(CodeChainCallRPCResponse {
        inner_response: value,
    })
}

fn hardware_get(context: &HandlerContext) -> RPCResult<HardwareInfo> {
    response(context.hardware_service.get())
}
