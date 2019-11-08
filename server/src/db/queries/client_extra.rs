use super::super::types::DBConnection;
use super::super::ClientExtra;

pub fn get(conn: &DBConnection, node_name: &str) -> postgres::Result<Option<ClientExtra>> {
    ctrace!("Query client extra by name {}", node_name);

    let rows = conn.query("SELECT * FROM client_extra WHERE name=$1;", &[&node_name])?;
    if rows.is_empty() {
        return Ok(None)
    }
    let row = rows.get(0);
    Ok(Some(ClientExtra {
        prev_env: row.get("prev_env"),
        prev_args: row.get("prev_args"),
    }))
}

pub fn upsert(conn: &DBConnection, node_name: &str, client_extra: &ClientExtra) -> postgres::Result<()> {
    ctrace!("Upsert client extra {:?}", client_extra);
    let result = conn.execute(
        "INSERT INTO client_extra (name, prev_env, prev_args) VALUES ($1, $2, $3) \
         ON CONFLICT (name) DO UPDATE \
         SET prev_env=excluded.prev_env, \
         prev_args=excluded.prev_args",
        &[&node_name, &client_extra.prev_env, &client_extra.prev_args],
    )?;
    ctrace!("Upsert result {}", result);
    Ok(())
}
