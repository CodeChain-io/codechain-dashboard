use std::net::SocketAddr;
use std::sync::mpsc::{channel, SendError, Sender};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::thread;
use std::vec::Vec;

use super::super::jsonrpc;
use super::agent::{Agent, AgentSender, State as AgentState};

pub struct State {
    agents: Vec<(i32, AgentSender)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }

    pub fn get_agent_info(&self) -> Vec<AgentState> {
        let agent_states = self.agents.iter().map(|(_, agent)| *agent.read_state()).collect();
        agent_states
    }
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
    state: Arc<RwLock<State>>,
}

impl ServiceSender {
    pub fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        self.sender.send(message)
    }

    pub fn read_state(&self) -> RwLockReadGuard<State> {
        self.state.read().expect("Should success read service state")
    }

    pub fn get_agent(&self, address: SocketAddr) -> Option<AgentSender> {
        let state = self.state.read().expect("Should access read service state");
        let find_result = state.agents.iter().find(|(_, agent)| {
            let agent_state = agent.read_state();
            match agent_state.address() {
                None => false,
                Some(agent_address) => agent_address == address,
            }
        });

        find_result.map(|(_, agent)| agent.clone())
    }
}

pub struct Service {
    state: Arc<RwLock<State>>,
    next_id: i32,
    sender: ServiceSender,
}

pub enum Message {
    InitializeAgent(jsonrpc::Context),
    AddAgent(i32, AgentSender),
    RemoveAgent(i32),
}

impl Service {
    pub fn run_thread() -> ServiceSender {
        let (tx, rx) = channel();
        let state = Arc::new(RwLock::new(State::new()));
        let service_sender = ServiceSender {
            sender: tx.clone(),
            state: state.clone(),
        };

        let mut service = Service::new(service_sender.clone(), state);

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

        service_sender
    }

    fn new(sender: ServiceSender, state: Arc<RwLock<State>>) -> Self {
        Service {
            state,
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
        let mut state = self.state.write().expect("Should get state");
        state.agents.push((id, agent_sender));
        cdebug!("Agent {} is added to AgentService", id);
    }

    fn remove_agent(&mut self, id: i32) {
        let mut state = self.state.write().expect("Should get state");

        let agent_index = state.agents.iter().position(|(iter_id, _)| *iter_id == id);
        if agent_index.is_none() {
            cerror!("Cannot find agent {} to delete", id);
            return
        }
        state.agents.remove(agent_index.unwrap());
        cdebug!("Agent {} is removed from AgentService", id);
    }
}
