use serde_json;
use serde_json::Value;

use super::db;
use super::frontend;
use super::jsonrpc;

pub struct EventPropagator {
    frontend_service: frontend::ServiceSender,
}

impl EventPropagator {
    pub fn new(frontend_service: frontend::ServiceSender) -> Self {
        EventPropagator {
            frontend_service,
        }
    }
}

impl db::EventSubscriber for EventPropagator {
    fn on_event(&self, event: db::Event) {
        match event {
            db::Event::AgentUpdated {
                before,
                after,
            } => {
                let mut diff = json!({
                    "name": after.name,
                });

                if before.is_none() {
                    diff["address"] = serde_json::to_value(after.address).unwrap();
                    diff["status"] = serde_json::to_value(after.status).unwrap();
                    diff["peers"] = serde_json::to_value(after.peers).unwrap();
                    diff["bestBlockId"] = serde_json::to_value(after.best_block_id).unwrap();
                    diff["version"] = serde_json::to_value(after.version).unwrap();
                } else {
                    let before = before.unwrap();
                    if before == after {
                        return
                    }

                    if before.address != after.address {
                        diff["address"] = serde_json::to_value(after.address).unwrap();
                    }
                    if before.status != after.status {
                        diff["status"] = serde_json::to_value(after.status).unwrap();
                    }
                    if before.peers != after.peers {
                        diff["peers"] = serde_json::to_value(after.peers).unwrap();
                    }
                    if before.best_block_id != after.best_block_id {
                        diff["bestBlockId"] = serde_json::to_value(after.best_block_id).unwrap();
                    }
                    if before.version != after.version {
                        diff["version"] = serde_json::to_value(after.version).unwrap();
                    }
                }

                let message = jsonrpc::serialize_notification(
                    "dashboard_updated",
                    json!({
                        "nodes": [diff.clone()]
                    }),
                );

                self.frontend_service.send(frontend::Message::SendEvent(message)).expect("Should success send event");
                let message = jsonrpc::serialize_notification("node_updated", diff);
                self.frontend_service.send(frontend::Message::SendEvent(message)).expect("Should success send event");
            }
            db::Event::ConnectionChanged {
                added,
                removed,
            } => {
                let collection_added: Vec<Value> = added
                    .iter()
                    .map(|(first, second)| {
                        json!({
                            "nodeA": first,
                            "nodeB": second,
                        })
                    })
                    .collect();
                let collection_removed: Vec<Value> = removed
                    .iter()
                    .map(|(first, second)| {
                        json!({
                            "nodeA": first,
                            "nodeB": second,
                        })
                    })
                    .collect();
                let message = jsonrpc::serialize_notification(
                    "dashboard_updated",
                    json!({
                        "connectionsAdded": collection_added,
                        "connectionsRemoved": collection_removed,
                    }),
                );

                self.frontend_service.send(frontend::Message::SendEvent(message)).expect("Should success send event");
            }
        }
    }
}
