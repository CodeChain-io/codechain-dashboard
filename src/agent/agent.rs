use std::net::SocketAddr;
use std::ops::Drop;
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard};
use std::thread;
use std::time::Duration;

use jsonrpc_core::Output;
use serde_json;
use serde_json::Value;
use ws::CloseCode as WSCloseCode;

use super::super::common_rpc_types::{
    BlockId, HardwareInfo, NodeName, NodeStatus, NodeVersion, ShellStartCodeChainRequest, ShellUpdateCodeChainRequest,
};
use super::super::db;
use super::super::jsonrpc;
use super::super::rpc::RPCResult;
use super::codechain_rpc::CodeChainRPC;
use super::service::{Message as ServiceMessage, ServiceSender};
use super::types::{AgentGetInfoResponse, CodeChainCallRPCResponse};

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Initializing,
    Normal {
        name: NodeName,
        address: Option<SocketAddr>,
        status: NodeStatus,
    },
    Stop {
        name: NodeName,
        address: Option<SocketAddr>,
        status: NodeStatus,
        cause: StopCause,
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StopCause {
    AlreadyConnected,
}

impl State {
    pub fn new() -> Self {
        State::Initializing
    }

    //    pub fn status(&self) -> Option<NodeStatus> {
    //        match self {
    //            State::Initializing => None,
    //            State::Normal {
    //                status,
    //                ..
    //            } => Some(*status),
    //            State::Stop {
    //                status,
    //                ..
    //            } => Some(*status),
    //        }
    //    }

    //    pub fn address(&self) -> Option<SocketAddr> {
    //        match self {
    //            State::Initializing => None,
    //            State::Normal {
    //                address,
    //                ..
    //            } => *address,
    //        }
    //    }
    //
    pub fn name(&self) -> Option<NodeName> {
        match self {
            State::Initializing => None,
            State::Normal {
                name,
                ..
            } => Some(name.clone()),
            State::Stop {
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
    db_service: db::ServiceSender,
    codechain_rpc: CodeChainRPC,
}

pub enum AgentCleanupReason {
    Error(String),
    #[allow(dead_code)]
    Success,
    AlreadyConnected,
    Unexpected,
}

impl Agent {
    fn new(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        db_service: db::ServiceSender,
    ) -> Self {
        let state = Arc::new(RwLock::new(State::new()));
        let sender = AgentSender::new(jsonrpc_context, Arc::clone(&state));
        Self {
            id,
            state,
            sender: sender.clone(),
            service_sender,
            closed: false,
            db_service,
            codechain_rpc: CodeChainRPC::new(sender),
        }
    }

    pub fn run_thread(
        id: i32,
        jsonrpc_context: jsonrpc::Context,
        service_sender: ServiceSender,
        db_service: db::ServiceSender,
    ) -> AgentSender {
        let mut agent = Self::new(id, jsonrpc_context, service_sender, db_service);
        let sender = agent.sender.clone();

        thread::Builder::new()
            .name(format!("agent-{}", id))
            .spawn(move || match agent.run() {
                Ok(StopCause::AlreadyConnected) => {
                    agent.clean_up(AgentCleanupReason::AlreadyConnected);
                }
                Err(err) => {
                    cerror!("Agent failed : {}", err);
                    agent.clean_up(AgentCleanupReason::Error(err));
                }
            })
            .expect("Should success running agent thread");

        sender
    }

    fn run(&mut self) -> Result<StopCause, String> {
        cinfo!("Agent-{} started", self.id);

        self.update()?;
        if let State::Stop {
            cause,
            ..
        } = *self.state.read().unwrap()
        {
            return Ok(cause)
        }
        self.service_sender
            .send(ServiceMessage::AddAgent(self.id, self.sender.clone()))
            .map_err(|err| format!("AddAgent failed {}", err))?;

        loop {
            ctrace!("Agent-{} update", self.id);
            self.update()?;
            if let State::Stop {
                cause,
                ..
            } = *self.state.read().unwrap()
            {
                return Ok(cause)
            }
            thread::sleep(Duration::new(10, 0));
        }
    }

    fn update(&mut self) -> Result<(), String> {
        let info = self.sender.agent_get_info().map_err(|err| format!("{}", err))?;

        let mut state = self.state.write().expect("Should success getting agent state");
        let new_state = State::Normal {
            name: info.name.clone(),
            address: info.address,
            status: info.status,
        };

        if let State::Initializing = *state {
            let success = self
                .db_service
                .initialize_agent_query_result(db::AgentQueryResult {
                    name: info.name.clone(),
                    status: info.status,
                    address: info.address,
                    version: Some(NodeVersion {
                        version: String::new(),
                        hash: info.codechain_commit_hash,
                    }),
                    ..Default::default()
                })
                .map_err(|_| "DB timeout")?;

            if !success {
                *state = State::Stop {
                    name: info.name.clone(),
                    address: info.address,
                    status: info.status,
                    cause: StopCause::AlreadyConnected,
                };
                return Ok(())
            }

            *state = new_state;
            return Ok(())
        }

        let peers: Vec<SocketAddr> = self.codechain_rpc.get_peers(info.status)?;
        let best_block_id: Option<BlockId> = self.codechain_rpc.get_best_block_id(info.status)?;
        let codechain_version = self.codechain_rpc.version(info.status)?;
        let codechain_version_hash = self.codechain_rpc.commit_hash(info.status)?;
        let version = codechain_version.and_then(|version| {
            codechain_version_hash.map(|hash| NodeVersion {
                version,
                hash,
            })
        });
        let version = version.or(Some(NodeVersion {
            version: String::new(),
            hash: info.codechain_commit_hash,
        }));
        let pending_parcels = self.codechain_rpc.get_pending_parcels(info.status)?;
        let whitelist = self.codechain_rpc.get_whitelist(info.status)?;
        let blacklist = self.codechain_rpc.get_blacklist(info.status)?;
        let hardware = self.sender.hardware_get().map_err(|err| format!("Agent Update {}", err))?;

        ctrace!("Update state from {:?} to {:?}", state, new_state);
        self.db_service.update_agent_query_result(db::AgentQueryResult {
            name: info.name.clone(),
            status: info.status,
            address: info.address,
            peers,
            best_block_id,
            version,
            pending_parcels,
            whitelist,
            blacklist,
            hardware: Some(hardware),
        });
        *state = new_state;

        let logs = self.codechain_rpc.get_logs(info.status)?;
        if let Some(logs) = logs {
            self.db_service.write_logs(&info.name, logs);
        }

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
            AgentCleanupReason::AlreadyConnected => {
                (true, "An agent which has same name is already connected".to_string())
            }
            AgentCleanupReason::Success => (false, "".to_string()),
        };

        let send_result = self.service_sender.send(ServiceMessage::RemoveAgent(self.id));
        if let Err(error) = send_result {
            cerror!("Agent cleanup error {}", error);
        }

        let state = self.state.read().expect("Should success read");
        if let State::Normal {
            name,
            address,
            ..
        } = state.clone()
        {
            self.db_service.update_agent_query_result(db::AgentQueryResult {
                name,
                status: NodeStatus::Error,
                address,
                ..Default::default()
            });
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
    fn shell_update_codechain(&self, _req: ShellUpdateCodeChainRequest) -> RPCResult<()>;
    fn shell_get_codechain_log(&self) -> RPCResult<String>;
    fn agent_get_info(&self) -> RPCResult<AgentGetInfoResponse>;
    fn codechain_call_rpc_raw(&self, args: (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse>;
    fn codechain_call_rpc(&self, args: (String, Vec<Value>)) -> RPCResult<Output>;
    fn hardware_get(&self) -> RPCResult<HardwareInfo>;
}

impl SendAgentRPC for AgentSender {
    fn shell_start_codechain(&self, req: ShellStartCodeChainRequest) -> RPCResult<()> {
        jsonrpc::call_one_arg(self.jsonrpc_context.clone(), "shell_startCodeChain", req)?;
        Ok(())
    }

    fn shell_stop_codechain(&self) -> RPCResult<()> {
        jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "shell_stopCodeChain")?;
        Ok(())
    }

    fn shell_update_codechain(&self, args: ShellUpdateCodeChainRequest) -> RPCResult<()> {
        jsonrpc::call_one_arg(self.jsonrpc_context.clone(), "shell_updateCodeChain", args)?;
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

    fn codechain_call_rpc_raw(&self, args: (String, Vec<Value>)) -> RPCResult<CodeChainCallRPCResponse> {
        let result = jsonrpc::call_many_args(self.jsonrpc_context.clone(), "codechain_callRPC", args)?;
        Ok(result)
    }

    fn codechain_call_rpc(&self, args: (String, Vec<Value>)) -> RPCResult<Output> {
        let result: CodeChainCallRPCResponse =
            jsonrpc::call_many_args(self.jsonrpc_context.clone(), "codechain_callRPC", args)?;
        let output: Output = serde_json::from_value(result.inner_response)?;
        Ok(output)
    }

    fn hardware_get(&self) -> RPCResult<HardwareInfo> {
        let result = jsonrpc::call_no_arg(self.jsonrpc_context.clone(), "hardware_get")?;
        Ok(result)
    }
}
