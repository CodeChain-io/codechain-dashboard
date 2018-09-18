use std::net::SocketAddr;
use std::ops::Drop;
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;

use ws::CloseCode as WSCloseCode;

use super::super::common_rpc_types::{NodeStatus, ShellStartCodeChainRequest};
use super::super::jsonrpc;
use super::super::rpc::RPCResult;
use super::service::{Message as ServiceMessage, ServiceSender};
use super::types::AgentGetInfoResponse;

#[derive(Copy, Clone)]
pub enum State {
    Initializing,
    Normal {
        address: SocketAddr,
        status: NodeStatus,
    },
}

impl State {
    pub fn new() -> Self {
        State::Initializing
    }

    pub fn address(&self) -> Option<SocketAddr> {
        match self {
            State::Initializing => None,
            State::Normal {
                address,
                ..
            } => Some(*address),
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
}

pub enum AgentCleanupReason {
    Error(String),
    Success,
    Unexpected,
}

impl Agent {
    fn new(id: i32, jsonrpc_context: jsonrpc::Context, service_sender: ServiceSender) -> Self {
        let state = Arc::new(RwLock::new(State::new()));
        Self {
            id,
            sender: AgentSender::new(jsonrpc_context, Arc::clone(&state)),
            state,
            service_sender,
            closed: false,
        }
    }

    pub fn run_thread(id: i32, jsonrpc_context: jsonrpc::Context, service_sender: ServiceSender) -> AgentSender {
        let mut agent = Self::new(id, jsonrpc_context, service_sender);
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
            cdebug!("Agent-{} update", self.id);
            self.update()?;
            thread::sleep(Duration::new(1, 0));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        let info = self.sender.agent_get_info().map_err(|err| format!("{}", err))?;

        let mut state = self.state.write().expect("Should success getting agent state");
        *state = State::Normal {
            address: info.address,
            status: info.status,
        };
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
    fn agent_get_info(&self) -> RPCResult<AgentGetInfoResponse>;
}

impl SendAgentRPC for AgentSender {
    fn shell_start_codechain(&self, req: ShellStartCodeChainRequest) -> RPCResult<()> {
        jsonrpc::call(self.jsonrpc_context.clone(), "shell_startCodeChain", req)?;
        Ok(())
    }

    fn agent_get_info(&self) -> RPCResult<AgentGetInfoResponse> {
        let result: AgentGetInfoResponse = jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "agent_getInfo")?;
        Ok(result)
    }
}
