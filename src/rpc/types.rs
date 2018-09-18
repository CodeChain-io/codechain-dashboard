use std::sync::mpsc::{RecvError, SendError};

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

use super::super::process::Error as ProcessError;
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}

pub type RPCResult<T> = Result<Option<T>, RPCError>;

pub enum RPCError {
    Process(ProcessError),
    Internal(String),
}

pub fn response<T>(value: T) -> RPCResult<T> {
    Ok(Some(value))
}

const ERR_ALREADY_RUNNING: i64 = 1;
const ERR_ENV_PARSE: i64 = 2;
const ERR_PROCESS_INTERNAL: i64 = 2;
const ERR_CODECHAIN_NOT_RUNNING: i64 = 3;

impl RPCError {
    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        match self {
            RPCError::Internal(str) => Self::create_internal_rpc_error(str),
            RPCError::Process(ProcessError::AlreadyRunning) => {
                Self::create_rpc_error(ERR_ALREADY_RUNNING, "CodeChain instance is already running")
            }
            RPCError::Process(ProcessError::EnvParseError) => {
                Self::create_rpc_error(ERR_ENV_PARSE, "Invalid env string")
            }
            RPCError::Process(ProcessError::SubprocessError(err)) => {
                Self::create_rpc_error(ERR_PROCESS_INTERNAL, &format!("Process error occured {:?}", err))
            }
            RPCError::Process(ProcessError::NotRunning) => {
                Self::create_rpc_error(ERR_CODECHAIN_NOT_RUNNING, "CodeChain is not running now")
            }
            RPCError::Process(ProcessError::IO(err)) => {
                Self::create_rpc_error(ERR_PROCESS_INTERNAL, &format!("IO error occured {:?}", err))
            }
        }
    }

    fn create_rpc_error(code: i64, msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::ServerError(code));
        ret.message = msg.to_string();
        ret
    }

    fn create_internal_rpc_error(msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::InternalError);
        ret.data = Some(Value::String(msg.to_string()));
        ret
    }
}

impl From<ProcessError> for RPCError {
    fn from(err: ProcessError) -> Self {
        RPCError::Process(err)
    }
}

impl From<SerdeError> for RPCError {
    fn from(err: SerdeError) -> Self {
        RPCError::Internal(format!("Internal error about JSON serialize/deserialize : {:?}", err))
    }
}

impl From<RecvError> for RPCError {
    fn from(err: RecvError) -> Self {
        RPCError::Internal(format!("Internal error about thread : {:?}", err))
    }
}

impl<T> From<SendError<T>> for RPCError {
    fn from(err: SendError<T>) -> Self {
        RPCError::Internal(format!("Internal error about thread : {:?}", err))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeStatus {
    Run,
    Stop,
    Error,
    UFO,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGetInfoResponse {
    pub status: NodeStatus,
    pub address: SocketAddr,
}
