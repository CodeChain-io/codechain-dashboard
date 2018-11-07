use chrono;
use postgres;

use super::super::super::common_rpc_types::{NodeName, StructuredLog};

pub fn insert(conn: &postgres::Connection, node_name: &NodeName, logs: Vec<StructuredLog>) -> postgres::Result<()> {
    ctrace!("Add log {} : {:?}", node_name, logs);
    for log in logs {
        let datetime = chrono::DateTime::parse_from_rfc3339(&log.timestamp).unwrap();
        conn.execute(
            "INSERT INTO logs (name, level, target, message, timestamp) VALUES ($1, $2, $3, $4, $5)",
            &[node_name, &log.level, &log.target, &log.message, &datetime],
        )?;
    }
    Ok(())
}
