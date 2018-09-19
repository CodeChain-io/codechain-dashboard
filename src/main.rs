#[macro_use]
extern crate clap;
extern crate jsonrpc_core;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate core;
extern crate serde_json;
extern crate subprocess;
extern crate ws;

#[macro_use]
mod logger;
mod agent;
mod handler;
mod process;
mod rpc;
mod types;

use self::agent::run;
use types::AgentArgs;

fn main() {
    let yaml = load_yaml!("agent.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let codechain_dir = matches.value_of("codechain-dir").expect("codechain-dir is required option");
    let log_file_path = matches.value_of("log-file").expect("log-file is required option");
    let hub_url = matches.value_of("connect").expect("connect is required option");
    let codechain_address = matches.value_of("address").expect("address is required option");
    let codechain_address = codechain_address.parse().expect("codechain-address field's format is invalid");

    let args = AgentArgs {
        codechain_dir,
        log_file_path,
        hub_url,
        codechain_address,
    };
    run(args);
}
