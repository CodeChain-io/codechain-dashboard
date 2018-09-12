#[macro_use]
extern crate log;

extern crate jsonrpc_core;
extern crate serde;
extern crate serde_json;
extern crate ws;

#[macro_use]
mod logger;
mod handler;
mod rpc;

use std::cell::Cell;
use std::rc::Rc;

use ws::listen;

use self::handler::WebSocketHandler;
use self::logger::init as logger_init;
use self::rpc::router::Router;
use rpc::frontend::add_routing;
use std::sync::Arc;

fn main() {
    logger_init().expect("Logger should be initialized");

    let count = Rc::new(Cell::new(0));

    let mut routing_table = Arc::new(Router::new());
    add_routing(Arc::get_mut(&mut routing_table).unwrap());
    cinfo!("Listen on 3012 port");
    listen("127.0.0.1:3012", |out| WebSocketHandler {
        out,
        count: count.clone(),
        routing_table: routing_table.clone(),
    }).unwrap();
}
