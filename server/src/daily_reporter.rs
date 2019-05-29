use std::sync::Arc;
use std::thread;

use chrono;

use super::noti::Noti;

pub fn start(noti: Arc<Noti>) -> thread::JoinHandle<()> {
    let network_id = std::env::var("NETWORK_ID").unwrap();

    thread::Builder::new()
        .name("daily reporter".to_string())
        .spawn(move || {
            let mut current_date = chrono::Utc::now().date();

            loop {
                let new_date = chrono::Utc::now().date();
                if new_date != current_date {
                    send_daily_report(&network_id, Arc::clone(&noti));
                }
                current_date = new_date;
                thread::sleep(std::time::Duration::from_secs(1000));
            }
        })
        .unwrap()
}

pub fn send_daily_report(network_id: &str, noti: Arc<Noti>) {
    noti.info(network_id, "Daily report", "CodeChain Server is running")
}
