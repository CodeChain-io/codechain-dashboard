use super::super::types::DBConnection;

pub fn set_query_timeout(conn: &DBConnection) -> postgres::Result<()> {
    conn.execute("SET SESSION statement_timeout TO 2000", &[])?;
    Ok(())
}
