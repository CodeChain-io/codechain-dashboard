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
mod agent;

use self::agent::run;

fn main() {
    run();
}
