#[macro_use]
extern crate log;

extern crate jsonrpc_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate subprocess;
extern crate ws;

#[macro_use]
mod logger;
mod handler;
mod process;
mod rpc;
mod types;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use ws::listen;

use self::handler::WebSocketHandler;
use self::logger::init as logger_init;
use self::process::{Process, ProcessOption};
use self::rpc::api::add_routing;
use self::rpc::router::Router;
use self::types::HandlerContext;

fn main() {
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
