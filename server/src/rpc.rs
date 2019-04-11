use std::fmt;
use std::result::Result;

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

use super::db::Error as DBError;
use super::jsonrpc;

pub type RPCResponse<T> = Result<Option<T>, RPCError>;

pub type RPCResult<T> = Result<T, RPCError>;

#[derive(Debug)]
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

pub fn response<T>(value: T) -> RPCResponse<T> {
    Ok(Some(value))
}

const ERR_AGENT_NOT_FOUND: i64 = -1;

impl From<RPCError> for JSONRPCError {
    fn from(err: RPCError) -> Self {
        match err {
            RPCError::Internal(str) => RPCError::create_internal_rpc_error(str),
            RPCError::FromAgent(mut error) => {
                error.data = match error.data {
                    None => Some(json!("Error from agent")),
                    Some(inner_data) => Some(json!({
                        "message": "This error is from the agent",
                        "inner": inner_data,
                    })),
                };
                error
            }
            RPCError::FromDB(_) => RPCError::create_internal_rpc_error(err.to_string()),
            RPCError::AgentNotFound => RPCError::create_rpc_error(ERR_AGENT_NOT_FOUND, err.to_string()),
        }
    }
}

impl RPCError {
    fn create_internal_rpc_error(msg: String) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::InternalError);
        ret.data = Some(Value::String(msg));
        ret
    }

    fn create_rpc_error(code: i64, msg: String) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::ServerError(code));
        ret.message = msg;
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
