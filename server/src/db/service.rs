use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use postgres;
use postgres::TlsMode;

use super::super::common_rpc_types as rpc_type;
use super::super::common_rpc_types::{NodeName, NodeStatus, StructuredLog};
use super::event::{Event, EventSubscriber};
use super::queries;
use super::types::{AgentExtra, AgentQueryResult, Connection, Connections, Error as DBError, Log, LogQueryParams};
use util;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeAgent(Box<AgentQueryResult>, Sender<bool>),
    UpdateAgent(Box<AgentQueryResult>),
    GetAgent(NodeName, Sender<Option<AgentQueryResult>>),
    GetAgents(Sender<Vec<AgentQueryResult>>),
    GetConnections(Sender<Vec<rpc_type::Connection>>),
    SaveStartOption(NodeName, String, String),
    GetAgentExtra(NodeName, Sender<Option<AgentExtra>>),
    GetLogs(LogQueryParams, Sender<Vec<Log>>),
    WriteLogs(NodeName, Vec<StructuredLog>),
    GetLogTargets(Sender<Vec<String>>),
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
}

struct State {
    agent_query_result: HashMap<NodeName, AgentQueryResult>,
    connection: Connections,
}

impl State {
    pub fn new() -> Self {
        Self {
            agent_query_result: HashMap::new(),
            connection: Connections::new(),
        }
    }
}

pub struct Service {
    state: State,
    event_subscriber: Box<EventSubscriber>,
    db_conn: postgres::Connection,
}

pub struct ServiceNewArg {
    pub event_subscriber: Box<EventSubscriber>,
    pub db_user: String,
    pub db_password: String,
}

impl Service {
    fn new(
        ServiceNewArg {
            event_subscriber,
            db_user,
            db_password,
        }: ServiceNewArg,
    ) -> Self {
        let conn_uri = format!("postgres://{}:{}@localhost", db_user, db_password);

        let conn = postgres::Connection::connect(conn_uri, TlsMode::None).unwrap();
        queries::config::set_query_timeout(&conn).unwrap();

        Self {
            state: State::new(),
            event_subscriber,
            db_conn: conn,
        }
    }

    pub fn run_thread(arg: ServiceNewArg) -> ServiceSender {
        let (tx, rx) = channel();
        let service_sender = ServiceSender::new(tx.clone());

        let mut service = Service::new(arg);

        thread::Builder::new()
            .name("db service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::InitializeAgent(agent_query_result, callback) => {
                            service.initialize_agent(&agent_query_result, callback);
                        }
                        Message::UpdateAgent(agent_query_result) => {
                            service.update_agent(*agent_query_result);
                        }
                        Message::GetAgent(node_name, callback) => {
                            service.get_agent(&node_name, callback);
                        }
                        Message::GetAgents(callback) => {
                            service.get_agents(callback);
                        }
                        Message::GetConnections(callback) => {
                            service.get_connections(callback);
                        }
                        Message::SaveStartOption(node_name, env, args) => {
                            util::log_error(&node_name, service.save_start_option(node_name.clone(), &env, &args));
                        }
                        Message::GetAgentExtra(node_name, callback) => {
                            util::log_error(&node_name, service.get_agent_extra(&node_name, callback));
                        }
                        Message::GetLogs(params, callback) => {
                            let result = service.get_logs(params, callback);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                        Message::WriteLogs(node_name, logs) => {
                            let result = service.write_logs(&node_name, logs);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                        Message::GetLogTargets(callback) => {
                            let result = service.get_log_targets(callback);
                            if let Err(err) = result {
                                cerror!("Error at {}", err);
                            }
                        }
                    }
                }
            })
            .expect("Should success running db service thread");

        service_sender
    }

    fn initialize_agent(&mut self, state: &AgentQueryResult, callback: Sender<bool>) {
        let name = state.name.clone();
        let before = match self.state.agent_query_result.entry(name) {
            Entry::Occupied(mut before) => before.into_mut(),
            Entry::Vacant(e) => {
                self.event_subscriber.on_event(Event::AgentUpdated {
                    before: None.into(),
                    after: state.clone().into(),
                });
                e.insert(state.clone());
                if let Err(err) = callback.send(true) {
                    cerror!("Cannot send callback : {}", err);
                }
                return
            }
        };

        if before.status != NodeStatus::Error {
            cinfo!(
                "Node {}({:?}) try to connect but a node with the same name already connected",
                state.name,
                before.status
            );
            if let Err(err) = callback.send(false) {
                cerror!("Cannot send callback : {}", err);
            }
            return
        }

        self.event_subscriber.on_event(Event::AgentUpdated {
            before: None.into(),
            after: state.clone().into(),
        });
        *before = state.clone();
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
                before: Some(before.clone()).into(),
                after: after.clone().into(),
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

    fn get_agent(&self, name: &str, callback: Sender<Option<AgentQueryResult>>) {
        let agent_query_result = self.state.agent_query_result.get(name);
        if let Err(err) = callback.send(agent_query_result.cloned()) {
            cerror!("Cannot call calback get_agent, name: {}\nerr: {}", name, err);
        }
    }

    fn get_agents(&self, callback: Sender<Vec<AgentQueryResult>>) {
        let states = self.state.agent_query_result.values().cloned().collect();
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

    fn save_start_option(&mut self, node_name: NodeName, env: &str, args: &str) -> Result<(), Box<error::Error>> {
        let before_extra = queries::agent_extra::get(&self.db_conn, &node_name)?;
        let mut extra = before_extra.clone().unwrap_or_default();

        extra.prev_env = env.to_string();
        extra.prev_args = args.to_string();

        queries::agent_extra::upsert(&self.db_conn, &node_name, &extra)?;

        self.event_subscriber.on_event(Event::AgentExtraUpdated {
            name: node_name,
            before: before_extra,
            after: extra,
        });

        Ok(())
    }

    fn get_agent_extra(&self, node_name: &str, callback: Sender<Option<AgentExtra>>) -> Result<(), Box<error::Error>> {
        let extra = queries::agent_extra::get(&self.db_conn, node_name)?;
        if let Err(err) = callback.send(extra) {
            cerror!("Callback error {}", err);
        }
        Ok(())
    }

    fn get_logs(&self, params: LogQueryParams, callback: Sender<Vec<Log>>) -> Result<(), Box<error::Error>> {
        let logs = queries::logs::search(&self.db_conn, params)?;
        callback.send(logs)?;
        Ok(())
    }

    fn write_logs(&self, node_name: &str, logs: Vec<StructuredLog>) -> Result<(), Box<error::Error>> {
        queries::logs::insert(&self.db_conn, node_name, logs)?;
        Ok(())
    }

    fn get_log_targets(&self, callback: Sender<Vec<String>>) -> Result<(), Box<error::Error>> {
        let targets = queries::logs::get_targets(&self.db_conn)?;
        callback.send(targets)?;
        Ok(())
    }
}

impl ServiceSender {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
        }
    }

    pub fn initialize_agent_query_result(&self, agent_query_result: AgentQueryResult) -> Result<bool, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::InitializeAgent(agent_query_result.into(), tx)).expect("Should success update agent");
        let result = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(result)
    }

    pub fn update_agent_query_result(&self, agent_query_result: AgentQueryResult) {
        self.sender.send(Message::UpdateAgent(agent_query_result.into())).expect("Should success update agent");
    }

    pub fn get_agent_query_result(&self, name: &str) -> Result<Option<AgentQueryResult>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgent(name.to_string(), tx)).expect("Should success send request");
        let agent_query_result = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(agent_query_result)
    }

    pub fn get_agents_state(&self) -> Result<Vec<AgentQueryResult>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgents(tx)).expect("Should success send request");
        let agents_state = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(agents_state)
    }

    pub fn get_connections(&self) -> Result<Vec<rpc_type::Connection>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetConnections(tx)).expect("Should success send request");
        let connections = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(connections)
    }

    pub fn save_start_option(&self, node_name: NodeName, env: &str, args: &str) {
        self.sender
            .send(Message::SaveStartOption(node_name, env.to_string(), args.to_string()))
            .expect("Should success send request");
    }

    pub fn get_agent_extra(&self, node_name: NodeName) -> Result<Option<AgentExtra>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetAgentExtra(node_name, tx)).expect("Should success send request");
        let agent_extra = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(agent_extra)
    }

    pub fn get_logs(&self, params: LogQueryParams) -> Result<Vec<Log>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetLogs(params, tx)).expect("Should success send request");
        let logs = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(logs)
    }

    pub fn write_logs(&self, node_name: NodeName, logs: Vec<StructuredLog>) {
        self.sender.send(Message::WriteLogs(node_name, logs)).expect("Should success send request");
    }

    pub fn get_log_targets(&self) -> Result<Vec<String>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetLogTargets(tx)).expect("Should success");
        let targets = rx.recv().map_err(|_| DBError::Timeout)?;
        Ok(targets)
    }
}
