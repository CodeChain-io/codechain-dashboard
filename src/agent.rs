use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::Arc;

use ws::connect;

use super::handler::WebSocketHandler;
use super::logger::init as logger_init;
use super::process::{Message as ProcessMessage, Process, ProcessOption};
use super::rpc::api::add_routing;
use super::rpc::router::Router;
use super::types::{AgentArgs, HandlerContext};

pub fn run(args: AgentArgs) {
    logger_init().expect("Logger should be initialized");

    let count = Rc::new(Cell::new(0));

    let mut router = Arc::new(Router::new());
    add_routing(Arc::get_mut(&mut router).unwrap());

    let process = Process::run_thread(ProcessOption {
        codechain_dir: args.codechain_dir.to_string(),
        log_file_path: args.log_file_path.to_string(),
    });

    let context = Arc::new(HandlerContext {
        codechain_address: args.codechain_address,
        name: args.name.to_string(),
        process: process.clone(),
    });

    cinfo!("Connect to {}", args.hub_url);
    if let Err(err) = connect(args.hub_url, move |out| WebSocketHandler {
        out,
        count: count.clone(),
        router: router.clone(),
        context: context.clone(),
    }) {
        cerror!("Error from websocket {}", err);
    }

    cinfo!("Close CodeChain");
    let (tx, rx) = channel();
    if let Err(err) = process.send(ProcessMessage::Quit {
        callback: tx,
    }) {
        cerror!("Error while closing CodeChain {}", err);
        return
    }
    match rx.recv() {
        Err(err) => cerror!("Error while closing CodeChain {}", err),
        Ok(Err(err)) => cerror!("Error while closing CodeChain {:?}", err),
        Ok(_) => {}
    }
}
