use std::sync::Arc;
use std::thread;

use chrono;

use super::db::ServiceSender as DBServiceSender;
use super::noti::Noti;

pub fn start(noti: Arc<Noti>, db_service: DBServiceSender) -> thread::JoinHandle<()> {
    let network_id = std::env::var("NETWORK_ID").unwrap();

    thread::Builder::new()
        .name("daily reporter".to_string())
        .spawn(move || {
            let mut current_date = chrono::Utc::now().date();

            loop {
                let new_date = chrono::Utc::now().date();
                if new_date != current_date {
                    send_daily_report(&network_id, Arc::clone(&noti), db_service.clone());
                }
                current_date = new_date;
                thread::sleep(std::time::Duration::from_secs(1000));
            }
        })
        .unwrap()
}

pub fn send_daily_report(network_id: &str, noti: Arc<Noti>, db_service: DBServiceSender) {
    let result = db_service.check_connection();
    let db_status = match result {
        Ok(_) => "DB is connected".to_string(),
        Err(err) => format!("DB connection has an error : {:?}", err),
    };
    let messages = ["CodeChain Server is running".to_string(), db_status];
    noti.info(network_id, "Daily report", &messages.join("\n"))
}
