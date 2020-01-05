#[macro_use]
mod logger;
mod client;
mod handler;
mod hardware_usage;
mod process;
mod rpc;
mod types;

use self::client::run;
use clap::load_yaml;
use types::ClientArgs;

fn main() {
    let yaml = load_yaml!("client.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let codechain_dir = matches.value_of("codechain-dir").expect("codechain-dir is required option");
    let log_file_path = matches.value_of("log-file").unwrap_or("codechain.log");
    let hub_url = matches.value_of("agent-hub-url").expect("agent-hub-url is required option");
    let codechain_address =
        matches.value_of("codechain-p2p-address").expect("codechain-p2p-address is required option");
    let codechain_address = codechain_address.parse().expect("codechain-p2p-address field's format is invalid");
    let name = matches.value_of("name").expect("name is required option");

    let args = ClientArgs {
        codechain_dir,
        log_file_path,
        hub_url,
        codechain_address,
        name,
    };
    run(&args);
}
