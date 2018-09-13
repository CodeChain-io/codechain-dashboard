use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use ws::listen;

use super::handler::WebSocketHandler;
use super::logger::init as logger_init;
use super::process::{Process, ProcessOption};
use super::rpc::api::add_routing;
use super::rpc::router::Router;
use super::types::HandlerContext;

pub fn run() {
    logger_init().expect("Logger should be initialized");

    let count = Rc::new(Cell::new(0));

    let mut routing_table = Arc::new(Router::new());
    add_routing(Arc::get_mut(&mut routing_table).unwrap());
    cinfo!("Listen on 4012 port");

    let process = Process::run_thread(ProcessOption {
        command: "codechain".to_string(),
        log_file: "codechain.stdout.log".to_string(),
    });

    let context = Arc::new(HandlerContext {
        process,
    });

    listen("127.0.0.1:4012", move |out| WebSocketHandler {
        out,
        count: count.clone(),
        routing_table: routing_table.clone(),
        context: context.clone(),
    }).unwrap();
}
