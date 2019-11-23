use std::{format, thread};

use chrono;
use postgres::{self, TlsMode};
use time;

use crate::db::queries::network_usage::remove_older_logs;

pub fn run(db_user: &str, db_password: &str) {
    let conn_uri = format!("postgres://{}:{}@localhost", db_user, db_password);
    let conn = postgres::Connection::connect(conn_uri, TlsMode::None).unwrap();
    let five_minutes = std::time::Duration::from_secs(5 * 60);

    thread::Builder::new()
        .name("cron-remove-network-usage".to_string())
        .spawn(move || loop {
            let current_date = chrono::Utc::now();
            let one_week_ago = current_date - time::Duration::days(7);
            if let Err(err) = remove_older_logs(&conn, one_week_ago) {
                cwarn!("Fail remove_older_logs: {:?}", err)
            }
            thread::sleep(five_minutes);
        })
        .expect("Should success listening frontend");
}
