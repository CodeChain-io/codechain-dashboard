use std::error::Error;
use std::fmt;
use std::result::Result;

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

use super::db::Error as DBError;
use super::jsonrpc;

pub type RPCResponse<T> = Result<Option<T>, RPCError>;

pub type RPCResult<T> = Result<T, RPCError>;

pub enum RPCError {
    Internal(String),
    FromAgent(JSONRPCError),
    FromDB(DBError),

    AgentNotFound,
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RPCError::Internal(err) => write!(f, "RPCError {}", err),
            RPCError::FromAgent(err) => write!(f, "JSONRPCError from Agent {:?}", err),
            RPCError::FromDB(err) => write!(f, "JSONRPCError from DB {:?}", err),
            RPCError::AgentNotFound => write!(f, "Agent not found"),
        }
    }
}

impl fmt::Debug for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for RPCError {}

pub fn response<T>(value: T) -> RPCResponse<T> {
    Ok(Some(value))
}

const ERR_AGENT_NOT_FOUND: i64 = -1;

impl RPCError {
    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        match self {
            RPCError::Internal(str) => Self::create_internal_rpc_error(str),
            RPCError::FromAgent(err) => {
                let mut error = err.clone();
                error.data = match error.data {
                    None => Some(json!("Error from agent")),
                    Some(inner_data) => Some(json!({
                        "message": "This error is from the agent",
                        "inner": inner_data,
                    })),
                };
                error
            }
            RPCError::FromDB(_err) => Self::create_internal_rpc_error(&self.to_string()),
            RPCError::AgentNotFound => Self::create_rpc_error(ERR_AGENT_NOT_FOUND, &format!("{}", self)),
        }
    }

    fn create_internal_rpc_error(msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::InternalError);
        ret.data = Some(Value::String(msg.to_string()));
        ret
    }

    fn create_rpc_error(code: i64, msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::ServerError(code));
        ret.message = msg.to_string();
        ret
    }
}

impl From<SerdeError> for RPCError {
    fn from(err: SerdeError) -> Self {
        RPCError::Internal(format!("Internal error about JSON serialize/deserialize : {:?}", err))
    }
}

impl From<jsonrpc::CallError> for RPCError {
    fn from(err: jsonrpc::CallError) -> Self {
        match err {
            jsonrpc::CallError::Response(jsonrpc_error) => RPCError::FromAgent(jsonrpc_error),
            _ => RPCError::Internal(format!("Internal error about jsonrpc call : {:?}", err)),
        }
    }
}

impl From<DBError> for RPCError {
    fn from(err: DBError) -> Self {
        RPCError::FromDB(err)
    }
}
