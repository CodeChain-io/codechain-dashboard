use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use super::super::common_rpc_types as rpc_type;
use super::super::common_rpc_types::{NodeName, NodeStatus};
use super::event::{Event, EventSubscriber};
use super::types::{AgentExtra, AgentQueryResult, Connection, Connections};

pub enum Message {
    InitializeAgent(AgentQueryResult, Sender<bool>),
    UpdateAgent(AgentQueryResult),
    GetAgent(NodeName, Sender<Option<AgentQueryResult>>),
    GetAgents(Sender<Vec<AgentQueryResult>>),
    GetConnections(Sender<Vec<rpc_type::Connection>>),
    SaveStartOption(NodeName, String, String),
    GetAgentExtra(NodeName, Sender<Option<AgentExtra>>),
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
}

struct State {
    agent_query_result: HashMap<NodeName, AgentQueryResult>,
    agent_extra: HashMap<NodeName, AgentExtra>,
    connection: Connections,
}

impl State {
    pub fn new() -> Self {
        Self {
            agent_query_result: HashMap::new(),
            agent_extra: HashMap::new(),
            connection: Connections::new(),
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
                        Message::InitializeAgent(agent_query_result, callback) => {
                            service.initialize_agent(agent_query_result, callback);
                        }
                        Message::UpdateAgent(agent_query_result) => {
                            service.update_agent(agent_query_result);
                        }
                        Message::GetAgent(node_name, callback) => {
                            service.get_agent(node_name, callback);
                        }
                        Message::GetAgents(callback) => {
                            service.get_agents(callback);
                        }
                        Message::GetConnections(callback) => {
                            service.get_connections(callback);
                        }
                        Message::SaveStartOption(node_name, env, args) => {
                            service.save_start_option(node_name, env, args);
                        }
                        Message::GetAgentExtra(node_name, callback) => {
                            service.get_agent_extra(node_name, callback);
                        }
                    }
                }
            })
            .expect("Should success running db service thread");

        service_sender
    }

    fn initialize_agent(&mut self, state: AgentQueryResult, callback: Sender<bool>) {
        let name = state.name.clone();
        if !self.state.agent_query_result.contains_key(&name) {
            self.event_subscriber.on_event(Event::AgentUpdated {
                before: None,
                after: state.clone(),
            });
            self.state.agent_query_result.insert(name, state);
            if let Err(err) = callback.send(true) {
                cerror!("Cannot send callback : {}", err);
            }
            return
        }

        let before = self.state.agent_query_result.get_mut(&name).unwrap();
        if before.status != NodeStatus::Error {
            cinfo!("Node {}({:?}) try to connect but a node with the same name already connected", name, before.status);
            if let Err(err) = callback.send(false) {
                cerror!("Cannot send callback : {}", err);
            }
            return
        }

        self.event_subscriber.on_event(Event::AgentUpdated {
            before: None,
            after: state.clone(),
        });
        *before = state;
        if let Err(err) = callback.send(true) {
            cerror!("Cannot send callback : {}", err);
        }
    }

    fn update_agent(&mut self, after: AgentQueryResult) {
        let name = after.name.clone();
        debug_assert_ne!(None, self.state.agent_query_result.get(&name));

        {
            let before = self.state.agent_query_result.get(&name).expect("Checked");

            let (added, removed) = self.state.connection.update(before, &after);
            if !added.is_empty() || !removed.is_empty() {
                self.event_subscriber.on_event(Event::ConnectionChanged {
                    added: added.iter().filter_map(|addrs| self.socket_addrs_to_name(addrs)).collect(),
                    removed: removed.iter().filter_map(|addrs| self.socket_addrs_to_name(addrs)).collect(),
                });
            }

            self.event_subscriber.on_event(Event::AgentUpdated {
                before: Some(before.clone()),
                after: after.clone(),
            });
        }

        let before = self.state.agent_query_result.get_mut(&name).expect("Checked");
        *before = after;
    }

    fn socket_addrs_to_name(&self, addrs: &Connection) -> Option<rpc_type::Connection> {
        let (first, second) = addrs;
        let first_name = self.socket_addr_to_name(first);
        let second_name = self.socket_addr_to_name(second);
        first_name.and_then(|first_name| second_name.map(|second_name| (first_name, second_name)))
    }

    fn socket_addr_to_name(&self, addr: &SocketAddr) -> Option<NodeName> {
        let find = self
            .state
            .agent_query_result
            .values()
            .find(|agent| agent.address.map(|agent_address| agent_address == *addr).unwrap_or(false));

        find.map(|agent| agent.name.clone())
    }

    fn get_agent(&self, name: NodeName, callback: Sender<Option<AgentQueryResult>>) {
        let agent_query_result = self.state.agent_query_result.get(&name);
        if let Err(err) = callback.send(agent_query_result.map(|state| state.clone())) {
            cerror!("Cannot call calback get_agent, name: {}\nerr: {}", name, err);
        }
    }

    fn get_agents(&self, callback: Sender<Vec<AgentQueryResult>>) {
        let states: Vec<AgentQueryResult> =
            self.state.agent_query_result.values().into_iter().map(|state| state.clone()).collect();
        if let Err(err) = callback.send(states) {
            cerror!("Callback error {}", err);
        }
    }

    fn get_connections(&self, callback: Sender<Vec<rpc_type::Connection>>) {
        let connections: Vec<Connection> = self.state.connection.get_all();
        let rpc_connections =
            connections.iter().filter_map(|connection| self.socket_addrs_to_name(connection)).collect();
        if let Err(err) = callback.send(rpc_connections) {
            cerror!("Callback error {}", err);
        }
    }

    fn save_start_option(&mut self, node_name: NodeName, env: String, args: String) {
        let extra_db = &mut self.state.agent_extra;
        let before_extra = extra_db.get(&node_name).cloned();
        let mut extra = before_extra.clone().unwrap_or(Default::default());

        extra.prev_env = env;
        extra.prev_args = args;

        let after_extra = extra.clone();
        extra_db.insert(node_name.clone(), extra);

        self.event_subscriber.on_event(Event::AgentExtraUpdated {
            name: node_name,
            before: before_extra,
            after: after_extra,
        });
    }

    fn get_agent_extra(&self, node_name: NodeName, callback: Sender<Option<AgentExtra>>) {
        let extra = self.state.agent_extra.get(&node_name).cloned();
        if let Err(err) = callback.send(extra) {
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

    pub fn initialize_agent_query_result(&self, agent_query_result: AgentQueryResult) -> bool {
        let (tx, rx) = channel();
        self.sender.send(Message::InitializeAgent(agent_query_result, tx)).expect("Should success update agent");
        let result = rx.recv().expect("Should success initialize_agent_query_result");
        result
    }

    pub fn update_agent_query_result(&self, agent_query_result: AgentQueryResult) {
        self.sender.send(Message::UpdateAgent(agent_query_result)).expect("Should success update agent");
    }

    pub fn get_agent_query_result(&self, name: &str) -> Option<AgentQueryResult> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgent(name.to_string(), tx)).expect("Should success send request");
        let agent_query_result = rx.recv().expect("Should success get_agent_query_result");
        agent_query_result
    }

    pub fn get_agents_state(&self) -> Vec<AgentQueryResult> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgents(tx)).expect("Should success send request");
        let agents_state = rx.recv().expect("Should success get_agents_state");
        agents_state
    }

    pub fn get_connections(&self) -> Vec<rpc_type::Connection> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetConnections(tx)).expect("Should success send request");
        let connections = rx.recv().expect("Should success get_connections");
        connections
    }

    pub fn save_start_option(&self, node_name: &NodeName, env: &str, args: &str) {
        self.sender
            .send(Message::SaveStartOption(node_name.clone(), env.to_string(), args.to_string()))
            .expect("Should success send request");
    }

    pub fn get_agent_extra(&self, node_name: &NodeName) -> Option<AgentExtra> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgentExtra(node_name.clone(), tx)).expect("Should success send request");
        let agent_extra = rx.recv().expect("Should success");
        agent_extra
    }
}
