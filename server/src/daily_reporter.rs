use std::sync::Arc;
use std::thread;

use chrono;

use super::agent::ServiceSender as AgentServiceSender;
use super::agent::State as AgentState;
use super::db::ServiceSender as DBServiceSender;
use super::noti::Noti;

pub fn start(
    noti: Arc<Noti>,
    db_service: DBServiceSender,
    agent_service: AgentServiceSender,
) -> thread::JoinHandle<()> {
    let network_id = std::env::var("NETWORK_ID").unwrap();

    thread::Builder::new()
        .name("daily reporter".to_string())
        .spawn(move || {
            let mut current_date = chrono::Utc::now().date();

            loop {
                let new_date = chrono::Utc::now().date();
                if new_date != current_date {
                    send_daily_report(&network_id, Arc::clone(&noti), db_service.clone(), agent_service.clone());
                }
                current_date = new_date;
                thread::sleep(std::time::Duration::from_secs(1000));
            }
        })
        .unwrap()
}

pub fn send_daily_report(
    network_id: &str,
    noti: Arc<Noti>,
    db_service: DBServiceSender,
    agent_service: AgentServiceSender,
) {
    let result = db_service.check_connection();
    let db_status = match result {
        Ok(_) => "DB is connected".to_string(),
        Err(err) => format!("DB connection has an error : {:?}", err),
    };
    let mut messages = vec!["CodeChain Server is running".to_string(), db_status];

    let agent_states = agent_service.get_agents_states();
    agent_service.reset_maximum_memory_usages();
    for agent_state in agent_states {
        match agent_state {
            AgentState::Initializing => {}
            AgentState::Normal {
                name,
                address,
                status,
                recent_update_result,
                maximum_memory_usage,
            } => {
                messages.push(format!("Agent: {}", name));
                messages.push(format!("  address: {:?}", address));
                messages.push(format!("  status: {:?}", status));
                if let Some(update_result) = recent_update_result {
                    messages.push(format!("  peer count: {}", update_result.number_of_peers));
                    messages.push(format!("  best block number: {:?}", update_result.best_block_number));
                    messages.push(format!("  available disk: {} MB", update_result.disk_usage.available / 1_000_000));
                }
                if let Some(maximum_memory_usage) = maximum_memory_usage {
                    let total_mb = maximum_memory_usage.total / 1_000_000;
                    let used_mb = (maximum_memory_usage.total - maximum_memory_usage.available) / 1_000_000;
                    messages.push(format!("  memory usage: {} MB / {} MB", used_mb, total_mb));
                }
            }
            AgentState::Stop {
                name,
                address,
                status,
                maximum_memory_usage,
                ..
            } => {
                messages.push(format!("Agent: {}", name));
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
