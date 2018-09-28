use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use super::super::common_rpc_types::NodeName;
use super::event::{Event, EventSubscriber};
use super::types::AgentState;

pub enum Message {
    UpdateAgent(AgentState),
    GetAgent(NodeName, Sender<Option<AgentState>>),
    GetAgents(Sender<Vec<AgentState>>),
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
}

struct State {
    agent_state: HashMap<NodeName, AgentState>,
}

impl State {
    pub fn new() -> Self {
        Self {
            agent_state: HashMap::new(),
        }
    }
}

pub struct Service {
    state: State,
    event_subscriber: Box<EventSubscriber>,
}

impl Service {
    fn new(event_subscriber: Box<EventSubscriber>) -> Self {
        Self {
            state: State::new(),
            event_subscriber,
        }
    }

    pub fn run_thread(event_subscriber: Box<EventSubscriber>) -> ServiceSender {
        let (tx, rx) = channel();
        let service_sender = ServiceSender::new(tx.clone());

        let mut service = Service::new(event_subscriber);

        thread::Builder::new()
            .name("db service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::UpdateAgent(agent_state) => {
                            service.update_agent(agent_state);
                        }
                        Message::GetAgent(node_name, callback) => {
                            service.get_agent(node_name, callback);
                        }
                        Message::GetAgents(callback) => {
                            service.get_agents(callback);
                        }
                    }
                }
            })
            .expect("Should success running db service thread");

        service_sender
    }

    fn update_agent(&mut self, after: AgentState) {
        let name = after.name.clone();
        let before_is_none = self.state.agent_state.get(&name).is_none();
        if before_is_none {
            self.event_subscriber.on_event(Event::AgentUpdated {
                before: None,
                after: after.clone(),
            });
            self.state.agent_state.insert(name, after);
            return
        }

        let before = self.state.agent_state.get_mut(&name).expect("Checked");

        if *before != after {
            self.event_subscriber.on_event(Event::AgentUpdated {
                before: Some(before.clone()),
                after: after.clone(),
            });
        }

        *before = after;
    }

    fn get_agent(&self, name: NodeName, callback: Sender<Option<AgentState>>) {
        let agent_state = self.state.agent_state.get(&name);
        if let Err(err) = callback.send(agent_state.map(|state| state.clone())) {
            cerror!("Cannot call calback get_agent, name: {}\nerr: {}", name, err);
        }
    }

    fn get_agents(&self, callback: Sender<Vec<AgentState>>) {
        let states: Vec<AgentState> = self.state.agent_state.values().into_iter().map(|state| state.clone()).collect();
        if let Err(err) = callback.send(states) {
            cerror!("Callback error {}", err);
        }
    }
}

impl ServiceSender {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
        }
    }

    pub fn update_agent_state(&self, agent_state: AgentState) {
        self.sender.send(Message::UpdateAgent(agent_state)).expect("Should success update agent");
    }

    pub fn get_agent_state(&self, name: &str) -> Option<AgentState> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgent(name.to_string(), tx)).expect("Should success send request");
        let agent_state = rx.recv().expect("Should success get_agent_state");
        agent_state
    }

    pub fn get_agents_state(&self) -> Vec<AgentState> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgents(tx)).expect("Should success send request");
        let agents_state = rx.recv().expect("Should success get_agents_state");
        agents_state
    }
}
