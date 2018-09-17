use std::error::Error;
use std::fmt;
use std::result::Result;

use jsonrpc;
use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

pub type RPCResponse<T> = Result<Option<T>, RPCError>;

pub type RPCResult<T> = Result<T, RPCError>;

pub enum RPCError {
    Internal(String),
}

impl fmt::Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RPCError::Internal(err) => write!(f, "RPCError {}", err),
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

impl RPCError {
    pub fn to_jsonrpc_error(&self) -> JSONRPCError {
        match self {
            RPCError::Internal(str) => Self::create_internal_rpc_error(str),
        }
    }

    fn create_internal_rpc_error(msg: &str) -> JSONRPCError {
        let mut ret = JSONRPCError::new(ErrorCode::InternalError);
        ret.data = Some(Value::String(msg.to_string()));
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
        RPCError::Internal(format!("Internal error about jsonrpc call : {:?}", err))
    }
}
