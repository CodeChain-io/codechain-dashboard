use std::sync::mpsc::{channel, SendError, Sender};
use std::sync::Arc;
use std::thread;
use std::vec::Vec;

use parking_lot::RwLock;

use super::super::db;
use super::super::jsonrpc;
use super::client::{Client, ClientSender, State as ClientState};
use crate::noti::Noti;

#[derive(Default)]
pub struct State {
    clients: Vec<(i32, ClientSender)>,
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

    pub fn get_client(&self, name: &str) -> Option<ClientSender> {
        let state = self.state.read();
        let find_result = state.clients.iter().find(|(_, client)| {
            let client_state = client.read_state();
            match client_state.name() {
                None => false,
                Some(client_name) => client_name == name,
            }
        });

        find_result.map(|(_, client)| client.clone())
    }

    pub fn get_clients_states(&self) -> Vec<ClientState> {
        let state = self.state.read();
        let mut result = Vec::new();
        for (_, client) in state.clients.iter() {
            let state = client.read_state().clone();
            result.push(state);
        }

        result
    }

    pub fn reset_maximum_memory_usages(&self) {
        let state = self.state.write();
        for (_, client) in state.clients.iter() {
            client.reset_maximum_memory_usage();
        }
    }
}

pub struct Service {
    state: Arc<RwLock<State>>,
    next_id: i32,
    sender: ServiceSender,
    db_service: db::ServiceSender,
}

pub enum Message {
    InitializeClient(jsonrpc::Context),
    AddClient(i32, ClientSender),
    RemoveClient(i32),
}

impl Service {
    pub fn run_thread(db_service: db::ServiceSender, noti: Arc<Noti>) -> ServiceSender {
        let (sender, rx) = channel();
        let state = Default::default();
        let service_sender = ServiceSender {
            sender,
            state: Arc::clone(&state),
        };

        let mut service = Service::new(service_sender.clone(), state, db_service);

        thread::Builder::new()
            .name("client service".to_string())
            .spawn(move || {
                for message in rx {
                    match message {
                        Message::InitializeClient(jsonrpc_context) => {
                            service.create_client(jsonrpc_context, Arc::clone(&noti));
                        }
                        Message::AddClient(id, client_sender) => {
                            service.add_client(id, client_sender);
                        }
                        Message::RemoveClient(id) => {
                            service.remove_client(id);
                        }
                    }
                }
            })
            .expect("Should success running client service thread");

        service_sender
    }

    fn new(sender: ServiceSender, state: Arc<RwLock<State>>, db_service: db::ServiceSender) -> Self {
        Service {
            state,
            next_id: 0_i32,
            sender,
            db_service,
        }
    }

    fn create_client(&mut self, jsonrpc_context: jsonrpc::Context, noti: Arc<Noti>) {
        let id = self.next_id;
        self.next_id += 1;
        Client::run_thread(id, jsonrpc_context, self.sender.clone(), self.db_service.clone(), noti);
        cdebug!("Client {} initialization starts", id);
    }

    fn add_client(&mut self, id: i32, client_sender: ClientSender) {
        let mut state = self.state.write();
        state.clients.push((id, client_sender));
        cdebug!("Client {} is added to ClientService", id);
    }

    fn remove_client(&mut self, id: i32) {
        let mut state = self.state.write();

        let client_index = state.clients.iter().position(|(iter_id, _)| *iter_id == id);
        if client_index.is_none() {
            cerror!("Cannot find client {} to delete", id);
            return
        }
        state.clients.remove(client_index.unwrap());
        cdebug!("Client {} is removed from ClientService", id);
    }
}
