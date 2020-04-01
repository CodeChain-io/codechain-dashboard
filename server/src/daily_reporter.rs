use super::client::{ServiceSender as ClientServiceSender, State as ClientState};
use super::db::ServiceSender as DBServiceSender;
use super::noti::Noti;
use std::sync::Arc;
use std::thread;

pub fn start(
    noti: Arc<Noti>,
    db_service: DBServiceSender,
    client_service: ClientServiceSender,
) -> thread::JoinHandle<()> {
    let network_id = std::env::var("NETWORK_ID").expect("NETWORK_ID environment variable is needed");

    thread::Builder::new()
        .name("daily reporter".to_string())
        .spawn(move || {
            let mut current_date = chrono::Utc::now().date();

            loop {
                let new_date = chrono::Utc::now().date();
                if new_date != current_date {
                    send_daily_report(&network_id, Arc::clone(&noti), db_service.clone(), client_service.clone());
                }
                current_date = new_date;
                thread::sleep(std::time::Duration::from_secs(1000));
            }
        })
        .unwrap()
}

enum DiskUsage {
    Unknown,
    Known {
        total: i64,
        per_disk: Vec<i64>,
    },
}

pub fn send_daily_report(
    network_id: &str,
    noti: Arc<Noti>,
    db_service: DBServiceSender,
    client_service: ClientServiceSender,
) {
    let result = db_service.check_connection();
    let db_status = match result {
        Ok(_) => "DB is connected".to_string(),
        Err(err) => format!("DB connection has an error : {:?}", err),
    };
    let mut messages = vec!["CodeChain Server is running".to_string(), db_status];

    let client_states = client_service.get_clients_states();
    client_service.reset_maximum_memory_usages();
    for client_state in client_states {
        match client_state {
            ClientState::Initializing => {}
            ClientState::Normal {
                name,
                address,
                status,
                recent_update_result,
                maximum_memory_usage,
            } => {
                messages.push(format!("Client: {}", name));
                messages.push(format!("  address: {:?}", address));
                messages.push(format!("  status: {:?}", status));
                if let Some(update_result) = recent_update_result {
                    let disk_usage = match (update_result.disk_usage, update_result.disk_usages) {
                        (_, Some(usages)) => DiskUsage::Known {
                            total: usages.iter().map(|usage| usage.available).sum::<i64>() / 1_000_000,
                            per_disk: usages.iter().map(|usage| usage.available).collect(),
                        },
                        (Some(usage), _) => DiskUsage::Known {
                            total: usage.available / 1_000_000,
                            per_disk: vec![usage.available],
                        },
                        _ => DiskUsage::Unknown,
                    };
                    messages.push(format!("  peer count: {}", update_result.number_of_peers));
                    messages.push(format!("  best block number: {:?}", update_result.best_block_number));
                    messages.push(match &disk_usage {
                        DiskUsage::Unknown => "  available disk: Unknown".to_string(),
                        DiskUsage::Known {
                            total,
                            per_disk,
                        } if per_disk.len() == 1 => format!("  available disk: {} MB", total),
                        DiskUsage::Known {
                            total,
                            per_disk,
                        } => {
                            let disk_usages_in_string: Vec<String> = per_disk
                                .iter()
                                .map(|available| available / 1_000_000i64)
                                .map(|available| available.to_string())
                                .collect();
                            format!("  available disk: {}({}) MB", total, disk_usages_in_string.join(" + "))
                        }
                    });
                }
                if let Some(maximum_memory_usage) = maximum_memory_usage {
                    let total_mb = maximum_memory_usage.total / 1_000_000;
                    let used_mb = (maximum_memory_usage.total - maximum_memory_usage.available) / 1_000_000;
                    messages.push(format!("  memory usage: {} MB / {} MB", used_mb, total_mb));
                }
            }
            ClientState::Stop {
                name,
                address,
                status,
                maximum_memory_usage,
                ..
            } => {
                messages.push(format!("Client: {}", name));
                messages.push(format!("  address: {:?}", address));
                messages.push(format!("  status: {:?}", status));

                if let Some(maximum_memory_usage) = maximum_memory_usage {
                    let total_mb = maximum_memory_usage.total / 1_000_000;
                    let used_mb = (maximum_memory_usage.total - maximum_memory_usage.available) / 1_000_000;
                    messages.push(format!("  memory usage: {} MB / {} MB", used_mb, total_mb));
                }
            }
        };
    }

    noti.info(network_id, "Daily report", &messages.join("\n"))
}
