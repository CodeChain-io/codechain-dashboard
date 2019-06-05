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
    create_peer_count_schema(&conn);
    create_network_usage_schema(&conn);
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
            bytes INTEGER NOT NULL,
            time_5min TIMESTAMP WITH TIME ZONE NOT NULL,
            time_hour TIMESTAMP WITH TIME ZONE NOT NULL,
            time_day TIMESTAMP WITH TIME ZONE NOT NULL
        )",
        &[],
    )
    .unwrap();

    cinfo!("Create network_usage_time_index");
    conn.execute("CREATE INDEX IF NOT EXISTS network_usage_time_index ON network_usage (time)", &[]).unwrap();
    conn.execute("CREATE INDEX IF NOT EXISTS network_usage_time_5min_index ON network_usage (name, time_5min)", &[])
        .unwrap();
    conn.execute("CREATE INDEX IF NOT EXISTS network_usage_time_hour_index ON network_usage (name, time_hour)", &[])
        .unwrap();
    conn.execute("CREATE INDEX IF NOT EXISTS network_usage_time_day_index ON network_usage (name, time_day)", &[])
        .unwrap();

    cinfo!("Create materialized views");
    conn.execute(
        "CREATE MATERIALIZED VIEW IF NOT EXISTS time_5min_report_view_materialized AS SELECT
        name,
        time_5min,
        CAST (SUM(bytes) AS REAL) AS value
        FROM network_usage
        WHERE time_5min > NOW() - interval '7 days'
        GROUP BY name, time_5min
        ORDER BY name, time_5min",
        &[],
    )
    .unwrap();

    conn.execute(
        "CREATE MATERIALIZED VIEW IF NOT EXISTS time_5min_avg_report_view_materialized AS SELECT
        network_usage.name,
        time_5min,
        CAST (SUM(bytes/greatest(peer_count.peer_count, 1)) AS REAL) AS value
        FROM network_usage
        LEFT JOIN peer_count ON (network_usage.time=peer_count.time AND network_usage.name=peer_count.name)
        WHERE time_5min > NOW() - interval '7 days'
        GROUP BY network_usage.name, time_5min
        ORDER BY network_usage.name, time_5min",
        &[],
    )
    .unwrap();

    conn.execute(
        "CREATE MATERIALIZED VIEW IF NOT EXISTS time_5min_extension_report_view_materialized AS SELECT
        name,
        extension,
        time_5min,
        CAST (SUM(bytes) AS REAL) AS value
        FROM network_usage
        WHERE time_5min > NOW() - interval '7 days'
        GROUP BY name, time_5min, network_usage.extension
        ORDER BY name, time_5min, network_usage.extension",
        &[],
    )
    .unwrap();

    conn.execute(
        "CREATE MATERIALIZED VIEW IF NOT EXISTS time_5min_peer_report_view_materialized AS SELECT
        name,
        target_ip,
        time_5min,
        CAST (SUM(bytes) AS REAL) AS value
        FROM network_usage
        WHERE time_5min > NOW() - interval '7 days'
        GROUP BY name, time_5min, network_usage.target_ip
        ORDER BY name, time_5min, network_usage.target_ip",
        &[],
    )
    .unwrap();
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
    conn.execute("CREATE INDEX IF NOT EXISTS peer_count_time_index ON peer_count (name, time)", &[]).unwrap();
}
