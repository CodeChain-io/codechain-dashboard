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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellUpdateCodeChainRequest {
    pub env: String,
    pub args: String,
    pub commit_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellGetCodeChainLogRequest {
    pub levels: Vec<String>,
}

pub type RPCResult<T> = Result<Option<T>, RPCError>;

pub enum RPCError {
    Process(ProcessError),
    Internal(String),

    // Will be returned as error response
    ErrorResponse(i64, String, Option<Value>),
}

pub fn response<T>(value: T) -> RPCResult<T> {
    Ok(Some(value))
}

const ERR_ALREADY_RUNNING: i64 = -10001;
const ERR_ENV_PARSE: i64 = -10002;
const ERR_CODECHAIN_UPDATING: i64 = -10003;
const ERR_PROCESS_INTERNAL: i64 = -32603;
const ERR_CODECHAIN_NOT_RUNNING: i64 = 0;

pub const ERR_NETWORK_ERROR: i64 = -10001;

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
            RPCError::Process(ProcessError::Updating) => {
                Self::create_rpc_error(ERR_CODECHAIN_UPDATING, "CodeChain is not updating now")
            }
            RPCError::Process(ProcessError::IO(err)) => {
                Self::create_rpc_error(ERR_PROCESS_INTERNAL, &format!("IO error occured {:?}", err))
            }
            RPCError::Process(ProcessError::CodeChainRPC(err)) => {
                Self::create_rpc_error(ERR_PROCESS_INTERNAL, &format!("Sending RPC to ChdeChain failed {}", err))
            }
            RPCError::Process(ProcessError::ShellError {
                exit_code,
                stdout,
                stderr,
            }) => Self::create_rpc_error(
                ERR_PROCESS_INTERNAL,
                &format!(
                    "Shell command has error exit code: {:?}\nstd out: {}\nstd error: {}\n",
                    exit_code, stdout, stderr
                ),
            ),
            RPCError::Process(ProcessError::Unknown(err)) => {
                Self::create_rpc_error(ERR_PROCESS_INTERNAL, &format!("Unknown error from process {}", err))
            }
            RPCError::ErrorResponse(code, message, value) => {
                Self::create_rpc_error_with_value(*code, message.clone(), value.clone())
            }
        }
    }

    fn create_rpc_error(code: i64, msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::ServerError(code));
        ret.message = msg.to_string();
        ret
    }

    fn create_rpc_error_with_value(code: i64, msg: String, value: Option<Value>) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::ServerError(code));
        ret.message = msg;
        ret.data = value;
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum NodeStatus {
    Starting,
    Run,
    Stop,
    Updating,
    Error,
    UFO,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGetInfoResponse {
    pub status: NodeStatus,
    pub name: String,
    pub address: Option<SocketAddr>,
    pub codechain_commit_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeChainCallRPCResponse {
    pub inner_response: Value,
}
