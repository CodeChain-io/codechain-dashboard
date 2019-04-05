use postgres;

use super::super::AgentExtra;

pub fn get(conn: &postgres::Connection, node_name: &str) -> postgres::Result<Option<AgentExtra>> {
    ctrace!("Query agent extra by name {}", node_name);

    let rows = conn.query("SELECT * FROM agent_extra WHERE name=$1;", &[&node_name])?;
    if rows.is_empty() {
        return Ok(None)
    }
    let row = rows.get(0);
    Ok(Some(AgentExtra {
        prev_env: row.get("prev_env"),
        prev_args: row.get("prev_args"),
    }))
}

pub fn upsert(conn: &postgres::Connection, node_name: &str, agent_extra: &AgentExtra) -> postgres::Result<()> {
    ctrace!("Upsert agent extra {:?}", agent_extra);
    let result = conn.execute(
        "INSERT INTO agent_extra (name, prev_env, prev_args) VALUES ($1, $2, $3) \
         ON CONFLICT (name) DO UPDATE \
         SET prev_env=excluded.prev_env, \
         prev_args=excluded.prev_args",
        &[&node_name, &agent_extra.prev_env, &agent_extra.prev_args],
    )?;
    ctrace!("Upsert result {}", result);
    Ok(())
}
