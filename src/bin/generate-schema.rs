extern crate postgres;

use postgres::{Connection, TlsMode};

fn main() {
    // FIXME: move to configuration file
    let user = "codechain-agent-hub";
    let password = "preempt-entreat-bell-chanson";
    let conn_uri = format!("postgres://{}:{}@localhost", user, password);
    let conn = Connection::connect(conn_uri, TlsMode::None).unwrap();
    let result = conn.query("SELECT 1", &[]).unwrap();

    println!("Query result is {:?}", result);
    println!("Hi I'm generate-schema");
}
