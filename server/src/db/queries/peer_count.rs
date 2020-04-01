use super::super::types::DBConnection;

pub fn insert(
    conn: &DBConnection,
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

pub fn remove_older_logs(conn: &DBConnection, time: chrono::DateTime<chrono::Utc>) -> postgres::Result<()> {
    ctrace!("Remove peer count older than {}", time);

    let result = conn.execute("DELETE FROM peer_count WHERE time<$1", &[&time])?;
    ctrace!("Delete result {}", result);
    Ok(())
}
