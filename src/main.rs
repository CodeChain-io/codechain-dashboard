extern crate ws;

mod handler;

use std::cell::Cell;
use std::rc::Rc;

use ws::listen;

use handler::WebSocketHandler;

fn main() {
    let count = Rc::new(Cell::new(0));
    println!("Listen on 3012 port");
    listen("127.0.0.1:3012", |out| { WebSocketHandler { out: out, count: count.clone() }}).unwrap();
}
