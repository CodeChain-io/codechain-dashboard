use std::result::Result;

use jsonrpc_core::types::{Error as JSONRPCError, ErrorCode};
use serde_json::{Error as SerdeError, Value};

pub type RPCResult<T> = Result<Option<T>, RPCError>;

pub enum RPCError {
    Internal(String),
}

pub fn response<T>(value: T) -> RPCResult<T> {
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

