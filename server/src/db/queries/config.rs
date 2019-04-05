use postgres;

pub fn set_query_timeout(conn: &postgres::Connection) -> postgres::Result<()> {
    conn.execute("SET SESSION statement_timeout TO 2000", &[])?;
    Ok(())
}
