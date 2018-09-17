use std::option::Option;

use jsonrpc_core::types::{Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Response, Success};
use serde_json;
use serde_json::Value;

use super::router::{Error as RouterError};

pub fn handle<F>(router: F, text: String) -> Option<String> where
    F: FnOnce(String, Value) -> Result<Option<Value>, RouterError> {
    let deserialized = serde_json::from_str(&text);
    let response: Option<Response> = match deserialized {
        Err(_) => Some(
            Failure {
                jsonrpc: None,
                id: Id::Null,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }.into(),
        ),
        Ok(Call::Invalid(id)) => Some(
            Failure {
                jsonrpc: None,
                id,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }.into(),
        ),
        Ok(Call::MethodCall(MethodCall {
            id,
            method,
            params,
            ..
        })) => {
            let value_params = serde_json::to_value(params).expect("Change to value always success");
            match router(method, value_params) {
                Ok(Some(value)) => Some(
                    Success {
                        jsonrpc: None,
                        result: value,
                        id,
                    }.into(),
                ),
                Ok(None) => {
                    let mut error = JSONRPCError::new(ErrorCode::InternalError);
                    error.data = Some(serde_json::Value::String("API returns no value".to_string()));
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error,
                        }.into(),
                    )
                }
                Err(RouterError::MethodNotFound) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: JSONRPCError::new(ErrorCode::MethodNotFound),
                    }.into(),
                ),
                Err(RouterError::RPC(err)) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: err.to_jsonrpc_error(),
                    }.into(),
                ),
            }
        }
        Ok(Call::Notification(_)) => None,
    };
    response.map(|response| serde_json::to_string(&response).expect("Should success serialize"))
}

pub fn invalid_format() -> String {
    serde_json::to_string(&Failure {
        jsonrpc: None,
        id: Id::Null,
        error: JSONRPCError::new(ErrorCode::ParseError),
    }).expect("Should success serialize")
}
