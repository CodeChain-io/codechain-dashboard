#[macro_use]
extern crate codechain_agent_hub as chub;
extern crate postgres;
#[macro_use]
extern crate log;

use chub::logger_init;
use postgres::{Connection, TlsMode};

fn main() {
    logger_init().expect("Logger should be initialized");

    // FIXME: move to configuration file
    let user = "codechain-agent-hub";
    let password = "preempt-entreat-bell-chanson";
    let conn_uri = format!("postgres://{}:{}@localhost", user, password);
    let conn = Connection::connect(conn_uri, TlsMode::None).unwrap();

    create_agent_extra_schema(&conn);
    create_logs_schema(&conn);
    create_network_usage_schema(&conn);
    create_peer_count_schema(&conn)
}

fn create_agent_extra_schema(conn: &Connection) {
    cinfo!("Create agent_extra table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agent_extra (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL UNIQUE,
        prev_env VARCHAR NOT NULL,
        prev_args VARCHAR NOT NULL
    )",
        &[],
    )
    .unwrap();
}

fn create_logs_schema(conn: &Connection) {
    cinfo!("Create logs table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
        id BIGSERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        level VARCHAR NOT NULL,
        target VARCHAR NOT NULL,
        thread_name VARCHAR NOT NULL,
        message VARCHAR NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL
    )",
        &[],
    )
    .unwrap();

    cinfo!("Create logs_timestamp index");
    conn.execute("CREATE INDEX IF NOT EXISTS logs_timestamp ON logs (timestamp)", &[]).unwrap();

    cinfo!("Create logs_target index");
    conn.execute("CREATE INDEX IF NOT EXISTS logs_targets ON logs (target)", &[]).unwrap();
}

fn create_network_usage_schema(conn: &Connection) {
    cinfo!("Create network_usage table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS network_usage (
            id BIGSERIAL PRIMARY KEY,
            time TIMESTAMP WITH TIME ZONE NOT NULL,
            name VARCHAR NOT NULL,
            extension VARCHAR NOT NULL,
            target_ip VARCHAR NOT NULL,
            bytes INTEGER NOT NULL
        )",
        &[],
    )
    .unwrap();

    cinfo!("Create network_usage_time_index");
    conn.execute("CREATE INDEX IF NOT EXISTS network_usage_time_index ON network_usage (time)", &[]).unwrap();
}

fn create_peer_count_schema(conn: &Connection) {
    cinfo!("Create peer_count table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS peer_count (
            id BIGSERIAL PRIMARY KEY,
            time TIMESTAMP WITH TIME ZONE NOT NULL,
            name VARCHAR NOT NULL,
            peer_count INTEGER NOT NULL
        )",
        &[],
    )
    .unwrap();

    cinfo!("Create peer_count_time_index");
    conn.execute("CREATE INDEX IF NOT EXISTS peer_count_time_index ON peer_count (time)", &[]).unwrap();
}
