use std::net::SocketAddr;
use std::ops::Drop;
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;

use serde_json;
use serde_json::Value;
use ws::CloseCode as WSCloseCode;

use super::super::common_rpc_types::{NodeStatus, ShellStartCodeChainRequest};
use super::super::frontend::service::{Message as FrontendServiceMessage, ServiceSender as FrontendServiceSender};
use super::super::frontend::types::DashboardNode;
use super::super::jsonrpc;
use super::super::rpc::RPCResult;
use super::service::{Message as ServiceMessage, ServiceSender};
use super::types::{AgentGetInfoResponse, CodeChainCallRPCResponse};

#[derive(Clone, PartialEq)]
pub enum State {
    Initializing,
    Normal {
        name: String,
        address: Option<SocketAddr>,
        status: NodeStatus,
    },
}

impl State {
    pub fn new() -> Self {
        State::Initializing
    }

    pub fn status(&self) -> Option<NodeStatus> {
        match self {
            State::Initializing => None,
            State::Normal {
                status,
                ..
            } => Some(*status),
        }
    }

    pub fn address(&self) -> Option<SocketAddr> {
        match self {
            State::Initializing => None,
            State::Normal {
                address,
                ..
            } => *address,
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            State::Initializing => None,
            State::Normal {
                name,
                ..
            } => Some(name.clone()),
        }
    }
}

#[derive(Clone)]
pub struct AgentSender {
    jsonrpc_context: jsonrpc::Context,
    state: Arc<RwLock<State>>,
}

impl AgentSender {
    pub fn new(jsonrpc_context: jsonrpc::Context, state: Arc<RwLock<State>>) -> Self {
        Self {
            jsonrpc_context,
            state,
        }
    }

    pub fn read_state(&self) -> RwLockReadGuard<State> {
        self.state.read().expect("Should success reading state")
    }
}

pub struct Agent {
    id: i32,
    sender: AgentSender,
    state: Arc<RwLock<State>>,
    service_sender: ServiceSender,
    closed: bool,
    frontend_service: FrontendServiceSender,
}

pub enum AgentCleanupReason {
    Error(String),
    Success,
    Unexpected,
}

impl Agent {
    fn new(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        frontend_service: FrontendServiceSender,
    ) -> Self {
        let state = Arc::new(RwLock::new(State::new()));
        Self {
            id,
            sender: AgentSender::new(jsonrpc_context, Arc::clone(&state)),
            state,
            service_sender,
            closed: false,
            frontend_service,
        }
    }

    pub fn run_thread(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        frontend_service: FrontendServiceSender,
    ) -> AgentSender {
        let mut agent = Self::new(id, jsonrpc_context, service_sender, frontend_service);
        let sender = agent.sender.clone();

        thread::Builder::new()
            .name(format!("agent-{}", id))
            .spawn(move || match agent.run() {
                Ok(_) => {
                    agent.clean_up(AgentCleanupReason::Success);
                }
                Err(err) => {
                    cerror!("Agent failed : {}", err);
                    agent.clean_up(AgentCleanupReason::Error(err));
                }
            })
            .expect("Should success running agent thread");

        sender
    }

    fn run(&mut self) -> Result<(), String> {
        cinfo!("Agent-{} started", self.id);

        self.update()?;
        self.service_sender
            .send(ServiceMessage::AddAgent(self.id, self.sender.clone()))
            .map_err(|err| format!("AddAgent failed {}", err))?;

        loop {
            ctrace!("Agent-{} update", self.id);
            self.update()?;
            thread::sleep(Duration::new(1, 0));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        let info = self.sender.agent_get_info().map_err(|err| format!("{}", err))?;

        let mut state = self.state.write().expect("Should success getting agent state");
        let new_state = State::Normal {
            name: info.name,
            address: info.address,
            status: info.status,
        };

        if new_state != *state {
            let mut diff = json!({});
            diff["name"] = serde_json::to_value(new_state.name()).unwrap();
            if state.address() != new_state.address() {
                diff["address"] = serde_json::to_value(new_state.address()).unwrap();
            }
            if state.status() != new_state.status() {
                diff["status"] = serde_json::to_value(new_state.status()).unwrap();
            }

            match *state {
                State::Initializing => {
                    let dashboard_node = DashboardNode::from_state(&new_state);
                    let message =
                        jsonrpc::serialize_notification("dashboard_updated", json!({ "nodes": [dashboard_node] }));
                    self.frontend_service
                        .send(FrontendServiceMessage::SendEvent(message))
                        .expect("Should success send event");
                }
                State::Normal {
                    ..
                } => {
                    let message = jsonrpc::serialize_notification(
                        "dashboard_updated",
                        json!({
                        "nodes": [diff.clone()]
                    }),
                    );
                    self.frontend_service
                        .send(FrontendServiceMessage::SendEvent(message))
                        .expect("Should success send event");
                    let message = jsonrpc::serialize_notification("node_updated", diff);
                    self.frontend_service
                        .send(FrontendServiceMessage::SendEvent(message))
                        .expect("Should success send event");
                }
            }

            cdebug!("Data updated, send notification");
        }

        *state = new_state;
        Ok(())
    }

    fn clean_up(&mut self, reason: AgentCleanupReason) {
        if self.closed {
            return
        }
        self.closed = true;

        let (is_error, error_msg) = match reason {
            AgentCleanupReason::Error(err) => {
                cerror!("Agent cleanuped because {}", err);
                (true, err)
            }
            AgentCleanupReason::Unexpected => {
                let err = "Unexpected cleanup";
                cerror!("Agent cleanuped because {}", err);
                (true, err.to_string())
            }
            AgentCleanupReason::Success => (false, "".to_string()),
        };

        let send_result = self.service_sender.send(ServiceMessage::RemoveAgent(self.id));
        if let Err(error) = send_result {
            cerror!("Agent cleanup error {}", error);
        }

        let ws_close_result = self.sender.jsonrpc_context.ws_sender.close_with_reason(
            if is_error {
                WSCloseCode::Error
            } else {
                WSCloseCode::Normal
            },
            error_msg,
        );

        if let Err(err) = ws_close_result {
            cerror!("Agent cleanup error {}", err);
        }
    }
}

impl Drop for Agent {
    fn drop(&mut self) {
        self.clean_up(AgentCleanupReason::Unexpected)
    }
}

pub trait SendAgentRPC {
    fn shell_start_codechain(&self, _req: ShellStartCodeChainRequest) -> RPCResult<()>;
    fn shell_stop_codechain(&self) -> RPCResult<()>;
    fn shell_get_codechain_log(&self) -> RPCResult<String>;
    fn agent_get_info(&self) -> RPCResult<AgentGetInfoResponse>;
    fn codechain_call_rpc(&self, args: (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse>;
}

impl SendAgentRPC for AgentSender {
    fn shell_start_codechain(&self, req: ShellStartCodeChainRequest) -> RPCResult<()> {
        jsonrpc::call(self.jsonrpc_context.clone(), "shell_startCodeChain", req)?;
        Ok(())
    }

    fn shell_stop_codechain(&self) -> RPCResult<()> {
        jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "shell_stopCodeChain")?;
        Ok(())
    }

    fn shell_get_codechain_log(&self) -> RPCResult<String> {
        let message = jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "shell_getCodeChainLog")?;
        Ok(message)
    }

    fn agent_get_info(&self) -> RPCResult<AgentGetInfoResponse> {
        let result: AgentGetInfoResponse = jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "agent_getInfo")?;
        Ok(result)
    }

    fn codechain_call_rpc(&self, args: (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse> {
        let result = jsonrpc::call(self.jsonrpc_context.clone(), "codechain_callRPC", args)?;
        Ok(result)
    }
}
