extern crate codechain_agent_hub as chub;
extern crate log;
extern crate postgres;

use chub::logger_init;
use postgres::{Connection, TlsMode};

fn main() {
    logger_init().expect("Logger should be initialized");

    // FIXME: move to configuration file
    let user = "codechain-agent-hub";
    let password = "preempt-entreat-bell-chanson";
    let conn_uri = format!("postgres://{}:{}@localhost", user, password);
    let conn = Connection::connect(conn_uri, TlsMode::None).unwrap();

    conn.execute("REFRESH MATERIALIZED VIEW time_5min_report_view_materialized", &[]).unwrap();
    conn.execute("REFRESH MATERIALIZED VIEW time_5min_avg_report_view_materialized", &[]).unwrap();
    conn.execute("REFRESH MATERIALIZED VIEW time_5min_extension_report_view_materialized", &[]).unwrap();
    conn.execute("REFRESH MATERIALIZED VIEW time_5min_peer_report_view_materialized", &[]).unwrap();
}
