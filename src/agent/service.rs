use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::vec::Vec;

use super::super::jsonrpc;
use super::agent::{Agent, AgentSender};

pub type ServiceSender = Sender<Message>;

pub struct Service {
    agents: Vec<(i32, AgentSender)>,
    next_id: i32,
    sender: Sender<Message>,
}

pub enum Message {
    InitializeAgent(jsonrpc::Context),
    AddAgent(i32, AgentSender),
    RemoveAgent(i32),
}

impl Service {
    pub fn run_thread() -> Sender<Message> {
        let (tx, rx) = channel();
        let mut service = Service::new(tx.clone());

        thread::Builder::new()
            .name("agent service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::InitializeAgent(jsonrpc_context) => {
                            service.create_agent(jsonrpc_context);
                        }
                        Message::AddAgent(id, agent_sender) => {
                            service.add_agent(id, agent_sender);
                        }
                        Message::RemoveAgent(id) => {
                            service.remove_agent(id);
                        }
                    }
                }
            })
            .expect("Should success running agent service thread");
        tx
    }

    fn new(sender: Sender<Message>) -> Self {
        Service {
            agents: Vec::new(),
            next_id: 0_i32,
            sender,
        }
    }

    fn create_agent(&mut self, jsonrpc_context: jsonrpc::Context) {
        let id = self.next_id;
        self.next_id += 1;
        Agent::run_thread(id, jsonrpc_context, self.sender.clone());
        cdebug!("Agent {} initialization starts", id);
    }

    fn add_agent(&mut self, id: i32, agent_sender: AgentSender) {
        self.agents.push((id, agent_sender));
        cdebug!("Agent {} is added to AgentService", id);
    }

    fn remove_agent(&mut self, id: i32) {
        let agent_index = self.agents.iter().position(|(iter_id, _)| *iter_id == id);
        if agent_index.is_none() {
            cerror!("Cannot find agent {} to delete", id);
            return
        }
        self.agents.remove(agent_index.unwrap());
        cdebug!("Agent {} is removed from AgentService", id);
    }
}
