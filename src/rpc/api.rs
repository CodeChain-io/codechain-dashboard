use std::sync::mpsc::channel;
use std::sync::Arc;

use super::super::process::Message as ProcessMessage;
use super::super::types::HandlerContext;
use super::router::Router;
use super::types::{Never, ShellStartCodeChainRequest};

pub fn add_routing(routing_table: &mut Router) {
    routing_table.add_route("ping", Box::new(ping as fn(Arc<HandlerContext>) -> String));
    routing_table.add_route(
        "shell_startCodeChain",
        Box::new(shell_start_code_chain as fn(Arc<HandlerContext>, ShellStartCodeChainRequest) -> Option<Never>),
    );
}

fn ping(_context: Arc<HandlerContext>) -> String {
    "pong".to_string()
}

fn shell_start_code_chain(context: Arc<HandlerContext>, req: ShellStartCodeChainRequest) -> Option<Never> {
    cinfo!("{}", req.env);
    cinfo!("{}", req.args);

    let (tx, rx) = channel();
    context
        .process
        .send(ProcessMessage::Run {
            env: req.env,
            args: req.args,
            callback: tx,
        })
        .expect("Should call process message");
    match rx.recv() {
        Ok(_) => None,
        Err(err) => {
            cerror!("Error {:?} occured", err);
            None
        }
    }
}
