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
use super::types::{ClientExtra, ClientQueryResult, Connection, Connections, Error as DBError, Log, LogQueryParams};
use common_rpc_types::{
    GraphCommonArgs, GraphNetworkOutAllAVGRow, GraphNetworkOutAllRow, GraphNetworkOutNodeExtensionRow,
    GraphNetworkOutNodePeerRow, NetworkUsage,
};
use util;

#[derive(Debug, Clone)]
pub enum Message {
    CheckConnection(Sender<Result<(), DBError>>),
    InitializeClient(Box<ClientQueryResult>, Sender<bool>),
    UpdateClient(Box<ClientQueryResult>),
    GetClient(NodeName, Sender<Option<ClientQueryResult>>),
    GetClients(Sender<Vec<ClientQueryResult>>),
    GetConnections(Sender<Vec<rpc_type::Connection>>),
    SaveStartOption(NodeName, String, String),
    GetClientExtra(NodeName, Sender<Option<ClientExtra>>),
    GetLogs(LogQueryParams, Sender<Vec<Log>>),
    WriteLogs(NodeName, Vec<StructuredLog>),
    GetLogTargets(Sender<Vec<String>>),
    WriteNetworkUsage(NodeName, NetworkUsage, chrono::DateTime<chrono::Utc>),
    WritePeerCount(NodeName, i32, chrono::DateTime<chrono::Utc>),
    GetGraphNetworkOutAll(GraphCommonArgs, Sender<Result<Vec<GraphNetworkOutAllRow>, DBError>>),
    GetGraphNetworkOutAllAVG(GraphCommonArgs, Sender<Result<Vec<GraphNetworkOutAllAVGRow>, DBError>>),
    GetGraphNetworkOutNodeExtension(
        NodeName,
        GraphCommonArgs,
        Sender<Result<Vec<GraphNetworkOutNodeExtensionRow>, DBError>>,
    ),
    GetGraphNetworkOutNodePeer(NodeName, GraphCommonArgs, Sender<Result<Vec<GraphNetworkOutNodePeerRow>, DBError>>),
}

#[derive(Clone)]
pub struct ServiceSender {
    sender: Sender<Message>,
}

#[derive(Default)]
struct State {
    client_query_result: HashMap<NodeName, ClientQueryResult>,
    connection: Connections,
}

pub struct Service {
    state: State,
    event_subscriber: Box<dyn EventSubscriber>,
    db_conn: postgres::Connection,
}

pub struct ServiceNewArg {
    pub event_subscriber: Box<dyn EventSubscriber>,
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
            state: State::default(),
            event_subscriber,
            db_conn: conn,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub fn run_thread(arg: ServiceNewArg) -> ServiceSender {
        let (tx, rx) = channel();
        let service_sender = ServiceSender::new(tx);

        let mut service = Service::new(arg);

        thread::Builder::new()
            .name("db service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::CheckConnection(callback) => {
                            service.check_connection(callback);
                        }
                        Message::InitializeClient(client_query_result, callback) => {
                            service.initialize_client(&client_query_result, callback);
                        }
                        Message::UpdateClient(client_query_result) => {
                            service.update_client(*client_query_result);
                        }
                        Message::GetClient(node_name, callback) => {
                            service.get_client(&node_name, callback);
                        }
                        Message::GetClients(callback) => {
                            service.get_clients(callback);
                        }
                        Message::GetConnections(callback) => {
                            service.get_connections(callback);
                        }
                        Message::SaveStartOption(node_name, env, args) => {
                            util::log_error(&node_name, service.save_start_option(node_name.clone(), &env, &args));
                        }
                        Message::GetClientExtra(node_name, callback) => {
                            util::log_error(&node_name, service.get_client_extra(&node_name, callback));
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
                        Message::WriteNetworkUsage(node_name, network_usage, time) => {
                            util::log_error(&node_name, service.write_network_usage(&node_name, network_usage, time));
                        }
                        Message::WritePeerCount(node_name, peer_count, time) => {
                            util::log_error(&node_name, service.write_peer_count(&node_name, peer_count, time));
                        }
                        Message::GetGraphNetworkOutAll(args, callback) => {
                            let result = service
                                .get_network_out_all_graph(args)
                                .map_err(|err| DBError::Internal(err.to_string()));
                            if let Err(callback_err) = callback.send(result) {
                                cerror!("Error at {}", callback_err);
                            }
                        }
                        Message::GetGraphNetworkOutAllAVG(args, callback) => {
                            let result = service
                                .get_network_out_all_avg_graph(args)
                                .map_err(|err| DBError::Internal(err.to_string()));
                            if let Err(callback_err) = callback.send(result) {
                                cerror!("Error at {}", callback_err);
                            }
                        }
                        Message::GetGraphNetworkOutNodeExtension(node_name, args, callback) => {
                            let result = service
                                .get_network_out_node_extension_graph(node_name, args)
                                .map_err(|err| DBError::Internal(err.to_string()));
                            if let Err(callback_err) = callback.send(result) {
                                cerror!("Error at {}", callback_err);
                            }
                        }
                        Message::GetGraphNetworkOutNodePeer(node_name, args, callback) => {
                            let result = service
                                .get_network_out_node_peer_graph(node_name, args)
                                .map_err(|err| DBError::Internal(err.to_string()));
                            if let Err(callback_err) = callback.send(result) {
                                cerror!("Error at {}", callback_err);
                            }
                        }
                    }
                }
            })
            .expect("Should success running db service thread");

        service_sender
    }

    fn check_connection(&self, callback: Sender<Result<(), DBError>>) {
        let result = self.db_conn.execute(&"SELECT 1", &[]).map_err(|err| DBError::Internal(err.to_string()));
        if let Err(err) = callback.send(result.map(|_| ())) {
            cerror!("Cannot send callback : {}", err);
        }
    }

    fn initialize_client(&mut self, state: &ClientQueryResult, callback: Sender<bool>) {
        let name = state.name.clone();
        let before = match self.state.client_query_result.entry(name) {
            Entry::Occupied(before) => before.into_mut(),
            Entry::Vacant(e) => {
                self.event_subscriber.on_event(Event::ClientUpdated {
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

        self.event_subscriber.on_event(Event::ClientUpdated {
            before: None.into(),
            after: state.clone().into(),
        });
        *before = state.clone();
        if let Err(err) = callback.send(true) {
            cerror!("Cannot send callback : {}", err);
        }
    }

    fn update_client(&mut self, after: ClientQueryResult) {
        let name = after.name.clone();
        debug_assert_ne!(None, self.state.client_query_result.get(&name));

        {
            let before = self.state.client_query_result.get(&name).expect("Checked");

            let (added, removed) = self.state.connection.update(before, &after);
            if !added.is_empty() || !removed.is_empty() {
                self.event_subscriber.on_event(Event::ConnectionChanged {
                    added: added.iter().filter_map(|addrs| self.socket_addrs_to_name(addrs)).collect(),
                    removed: removed.iter().filter_map(|addrs| self.socket_addrs_to_name(addrs)).collect(),
                });
            }

            self.event_subscriber.on_event(Event::ClientUpdated {
                before: Some(before.clone()).into(),
                after: after.clone().into(),
            });
        }

        let before = self.state.client_query_result.get_mut(&name).expect("Checked");
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
            .client_query_result
            .values()
            .find(|client| client.address.map(|client_address| client_address == *addr).unwrap_or(false));

        find.map(|client| client.name.clone())
    }

    fn get_client(&self, name: &str, callback: Sender<Option<ClientQueryResult>>) {
        let client_query_result = self.state.client_query_result.get(name);
        if let Err(err) = callback.send(client_query_result.cloned()) {
            cerror!("Cannot call calback get_client, name: {}\nerr: {}", name, err);
        }
    }

    fn get_clients(&self, callback: Sender<Vec<ClientQueryResult>>) {
        let states = self.state.client_query_result.values().cloned().collect();
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

    fn save_start_option(&mut self, node_name: NodeName, env: &str, args: &str) -> Result<(), Box<dyn error::Error>> {
        let before_extra = queries::client_extra::get(&self.db_conn, &node_name)?;
        let mut extra = before_extra.clone().unwrap_or_default();

        extra.prev_env = env.to_string();
        extra.prev_args = args.to_string();

        queries::client_extra::upsert(&self.db_conn, &node_name, &extra)?;

        self.event_subscriber.on_event(Event::ClientExtraUpdated {
            name: node_name,
            before: before_extra,
            after: extra,
        });

        Ok(())
    }

    fn get_client_extra(
        &self,
        node_name: &str,
        callback: Sender<Option<ClientExtra>>,
    ) -> Result<(), Box<dyn error::Error>> {
        let extra = queries::client_extra::get(&self.db_conn, node_name)?;
        if let Err(err) = callback.send(extra) {
            cerror!("Callback error {}", err);
        }
        Ok(())
    }

    fn get_logs(&self, params: LogQueryParams, callback: Sender<Vec<Log>>) -> Result<(), Box<dyn error::Error>> {
        let logs = queries::logs::search(&self.db_conn, params)?;
        callback.send(logs)?;
        Ok(())
    }

    fn write_logs(&self, node_name: &str, logs: Vec<StructuredLog>) -> Result<(), Box<dyn error::Error>> {
        queries::logs::insert(&self.db_conn, node_name, logs)?;
        Ok(())
    }

    fn get_log_targets(&self, callback: Sender<Vec<String>>) -> Result<(), Box<dyn error::Error>> {
        let targets = queries::logs::get_targets(&self.db_conn)?;
        callback.send(targets)?;
        Ok(())
    }

    fn write_network_usage(
        &self,
        node_name: &str,
        network_usage: NetworkUsage,
        time: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn error::Error>> {
        queries::network_usage::insert(&self.db_conn, node_name, network_usage, time)?;
        Ok(())
    }

    fn write_peer_count(
        &self,
        node_name: &str,
        peer_count: i32,
        time: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), Box<dyn error::Error>> {
        queries::peer_count::insert(&self.db_conn, node_name, peer_count, time)?;
        Ok(())
    }

    fn get_network_out_all_graph(
        &self,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutAllRow>, Box<dyn error::Error>> {
        let rows = queries::network_usage_graph::query_network_out_all(&self.db_conn, args)?;
        Ok(rows)
    }

    fn get_network_out_all_avg_graph(
        &self,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutAllRow>, Box<dyn error::Error>> {
        let rows = queries::network_usage_graph::query_network_out_all_avg(&self.db_conn, args)?;
        Ok(rows)
    }

    fn get_network_out_node_extension_graph(
        &self,
        node_name: NodeName,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutNodeExtensionRow>, Box<dyn error::Error>> {
        let rows = queries::network_usage_graph::query_network_out_node_extension(&self.db_conn, node_name, args)?;
        Ok(rows)
    }

    fn get_network_out_node_peer_graph(
        &self,
        node_name: NodeName,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutNodePeerRow>, Box<dyn error::Error>> {
        let rows = queries::network_usage_graph::query_network_out_node_peer(&self.db_conn, node_name, args)?;
        Ok(rows)
    }
}

impl ServiceSender {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
        }
    }

    pub fn check_connection(&self) -> Result<(), DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::CheckConnection(tx)).expect("Should success check connection");
        rx.recv()?
    }

    pub fn initialize_client_query_result(&self, client_query_result: ClientQueryResult) -> Result<bool, DBError> {
        let (tx, rx) = channel();
        self.sender
            .send(Message::InitializeClient(client_query_result.into(), tx))
            .expect("Should success update client");
        let result = rx.recv()?;
        Ok(result)
    }

    pub fn update_client_query_result(&self, client_query_result: ClientQueryResult) {
        self.sender.send(Message::UpdateClient(client_query_result.into())).expect("Should success update client");
    }

    pub fn get_client_query_result(&self, name: &str) -> Result<Option<ClientQueryResult>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetClient(name.to_string(), tx)).expect("Should success send request");
        let client_query_result = rx.recv()?;
        Ok(client_query_result)
    }

    pub fn get_clients_state(&self) -> Result<Vec<ClientQueryResult>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetClients(tx)).expect("Should success send request");
        let clients_state = rx.recv()?;
        Ok(clients_state)
    }

    pub fn get_connections(&self) -> Result<Vec<rpc_type::Connection>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetConnections(tx)).expect("Should success send request");
        let connections = rx.recv()?;
        Ok(connections)
    }

    pub fn save_start_option(&self, node_name: NodeName, env: &str, args: &str) {
        self.sender
            .send(Message::SaveStartOption(node_name, env.to_string(), args.to_string()))
            .expect("Should success send request");
    }

    pub fn get_client_extra(&self, node_name: NodeName) -> Result<Option<ClientExtra>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetClientExtra(node_name, tx)).expect("Should success send request");
        let client_extra = rx.recv()?;
        Ok(client_extra)
    }

    pub fn get_logs(&self, params: LogQueryParams) -> Result<Vec<Log>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetLogs(params, tx)).expect("Should success send request");
        let logs = rx.recv()?;
        Ok(logs)
    }

    pub fn write_logs(&self, node_name: NodeName, logs: Vec<StructuredLog>) {
        self.sender.send(Message::WriteLogs(node_name, logs)).expect("Should success send request");
    }

    pub fn get_log_targets(&self) -> Result<Vec<String>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetLogTargets(tx)).expect("Should success");
        let targets = rx.recv()?;
        Ok(targets)
    }

    pub fn write_network_usage(
        &self,
        node_name: NodeName,
        network_usage: NetworkUsage,
        time: chrono::DateTime<chrono::Utc>,
    ) {
        self.sender
            .send(Message::WriteNetworkUsage(node_name, network_usage, time))
            .expect("Should success send request");
    }

    pub fn write_peer_count(&self, node_name: NodeName, peer_count: i32, time: chrono::DateTime<chrono::Utc>) {
        self.sender.send(Message::WritePeerCount(node_name, peer_count, time)).expect("Should success send request");
    }

    pub fn get_network_out_all_graph(&self, args: GraphCommonArgs) -> Result<Vec<GraphNetworkOutAllRow>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetGraphNetworkOutAll(args, tx)).expect("Should success send request");
        rx.recv()?
    }

    pub fn get_network_out_all_avg_graph(
        &self,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutAllAVGRow>, DBError> {
        let (tx, rx) = channel();
        self.sender.send(Message::GetGraphNetworkOutAllAVG(args, tx)).expect("Should success send request");
        rx.recv()?
    }

    pub fn get_network_out_node_extension_graph(
        &self,
        node_name: NodeName,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutNodeExtensionRow>, DBError> {
        let (tx, rx) = channel();
        self.sender
            .send(Message::GetGraphNetworkOutNodeExtension(node_name, args, tx))
            .expect("should success send request");
        rx.recv()?
    }

    pub fn get_network_out_node_peer_graph(
        &self,
        node_name: NodeName,
        args: GraphCommonArgs,
    ) -> Result<Vec<GraphNetworkOutNodePeerRow>, DBError> {
        let (tx, rx) = channel();
        self.sender
            .send(Message::GetGraphNetworkOutNodePeer(node_name, args, tx))
            .expect("Should success send request");
        rx.recv()?
    }
}
