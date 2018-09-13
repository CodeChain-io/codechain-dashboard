use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use ws::connect;

use super::handler::WebSocketHandler;
use super::logger::init as logger_init;
use super::process::{Process, ProcessOption};
use super::rpc::api::add_routing;
use super::rpc::router::Router;
use super::types::{AgentArgs, HandlerContext};

pub fn run(args: AgentArgs) {
    logger_init().expect("Logger should be initialized");

    let count = Rc::new(Cell::new(0));

    let mut routing_table = Arc::new(Router::new());
    add_routing(Arc::get_mut(&mut routing_table).unwrap());

    let process = Process::run_thread(ProcessOption {
        codechain_dir: args.codechain_dir.to_string(),
        log_file_path: args.log_file_path.to_string(),
    });

    let context = Arc::new(HandlerContext {
        process,
    });

    cinfo!("Connect to {}", args.hub_url);
    connect(args.hub_url, move |out| WebSocketHandler {
        out,
        count: count.clone(),
        routing_table: routing_table.clone(),
        context: context.clone(),
    }).unwrap();
}
