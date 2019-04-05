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

    let table_names = get_all_table_names(&conn);
    cinfo!("Table names to delete are {:?}", table_names);

    for table_name in table_names {
        cinfo!("Drop table {}", &table_name);
        drop_table(&conn, &table_name);
    }
}

fn get_all_table_names(conn: &Connection) -> Vec<String> {
    let rows = conn
        .query(
            "SELECT table_name \
             FROM information_schema.tables \
             WHERE table_schema='public' \
             AND table_type='BASE TABLE'",
            &[],
        )
        .unwrap();
    rows.iter().map(|row| row.get(0)).collect()
}

fn drop_table(conn: &Connection, table_name: &str) {
    conn.execute(&format!("DROP TABLE {}", table_name), &[]).unwrap();
}
