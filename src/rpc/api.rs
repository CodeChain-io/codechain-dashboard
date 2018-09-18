use std::sync::mpsc::channel;
use std::sync::Arc;

use super::super::process::Message as ProcessMessage;
use super::super::types::HandlerContext;
use super::router::Router;
use super::types::{response, RPCResult, ShellStartCodeChainRequest};
use rpc::types::AgentGetInfoResponse;

pub fn add_routing(router: &mut Router) {
    router.add_route("ping", Box::new(ping as fn(Arc<HandlerContext>) -> RPCResult<String>));
    router.add_route(
        "shell_startCodeChain",
        Box::new(shell_start_code_chain as fn(Arc<HandlerContext>, (ShellStartCodeChainRequest,)) -> RPCResult<()>),
    );
    router
        .add_route("shell_stopCodeChain", Box::new(shell_stop_code_chain as fn(Arc<HandlerContext>) -> RPCResult<()>));
    router.add_route(
        "agent_getInfo",
        Box::new(agent_get_info as fn(Arc<HandlerContext>) -> RPCResult<AgentGetInfoResponse>),
    )
}

fn ping(_context: Arc<HandlerContext>) -> RPCResult<String> {
    response("pong".to_string())
}

fn shell_start_code_chain(context: Arc<HandlerContext>, req: (ShellStartCodeChainRequest,)) -> RPCResult<()> {
    let (req,) = req;
    cinfo!("{}", req.env);
    cinfo!("{}", req.args);

    let (tx, rx) = channel();
    context.process.send(ProcessMessage::Run {
        env: req.env,
        args: req.args,
        callback: tx,
    })?;
    let process_result = rx.recv()?;
    process_result?;
    response(())
}

fn shell_stop_code_chain(context: Arc<HandlerContext>) -> RPCResult<()> {
    let (tx, rx) = channel();
    context.process.send(ProcessMessage::Stop {
        callback: tx,
    })?;
    let process_result = rx.recv()?;
    process_result?;
    response(())
}

fn agent_get_info(context: Arc<HandlerContext>) -> RPCResult<AgentGetInfoResponse> {
    let (tx, rx) = channel();
    context.process.send(ProcessMessage::GetStatus {
        callback: tx,
    })?;
    let process_result = rx.recv()?;
    let node_status = process_result?;
    response(AgentGetInfoResponse {
        status: node_status,
        address: "127.0.0.1:3485".parse().unwrap(),
    })
}
