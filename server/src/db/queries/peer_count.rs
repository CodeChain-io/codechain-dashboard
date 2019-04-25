use chrono;
use postgres;

pub fn insert(
    conn: &postgres::Connection,
    node_name: &str,
    peer_count: i32,
    time: chrono::DateTime<chrono::Utc>,
) -> postgres::Result<()> {
    ctrace!("Add peer count of {}", node_name);

    conn.execute("INSERT INTO peer_count (time, name, peer_count) VALUES ($1, $2, $3)", &[
        &time,
        &node_name,
        &peer_count,
    ])?;
    Ok(())
}
