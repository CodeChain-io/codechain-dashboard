use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::option::Option;
use std::result::Result::{Err, Ok};
use std::sync::mpsc::{channel, RecvError, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::sync::PoisonError;
use std::time::Duration;

use jsonrpc_core::types::{
    Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Notification, Output, Params, Response, Success,
    Version,
};
use parking_lot::Mutex;
use rand;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_json::{Error as SerdeError, Value};

use super::router::Error as RouterError;
use super::ws::{Error as WSError, Message, Sender as WSSender};

pub fn handle<F>(router: F, text: String) -> Option<String>
where
    F: FnOnce(String, Value) -> Result<Option<Value>, RouterError>, {
    let deserialized = serde_json::from_str(&text);
    let response: Option<Response> = match deserialized {
        Err(_) => Some(
            Failure {
                jsonrpc: None,
                id: Id::Null,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }
            .into(),
        ),
        Ok(Call::Invalid(id)) => Some(
            Failure {
                jsonrpc: None,
                id,
                error: JSONRPCError::new(ErrorCode::ParseError),
            }
            .into(),
        ),
        Ok(Call::MethodCall(MethodCall {
            id,
            method,
            params,
            ..
        })) => {
            let value_params = serde_json::to_value(params.clone()).expect("Change to value always success");
            match router(method.clone(), value_params) {
                Ok(Some(value)) => Some(
                    Success {
                        jsonrpc: None,
                        result: value,
                        id,
                    }
                    .into(),
                ),
                Ok(None) => {
                    let mut error = JSONRPCError::new(ErrorCode::InternalError);
                    error.data = Some(serde_json::Value::String("API returns no value".to_string()));
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error,
                        }
                        .into(),
                    )
                }
                Err(RouterError::MethodNotFound) => Some(
                    Failure {
                        jsonrpc: None,
                        id,
                        error: JSONRPCError::new(ErrorCode::MethodNotFound),
                    }
                    .into(),
                ),
                Err(RouterError::RPC(err)) => {
                    cwarn!("Error while handlinig {}({:#?}) : {}", method, params, err);
                    Some(
                        Failure {
                            jsonrpc: None,
                            id,
                            error: err.to_jsonrpc_error(),
                        }
                        .into(),
                    )
                }
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
    })
    .expect("Should success serialize")
}

#[derive(Clone)]
pub struct Context {
    pub ws_sender: WSSender,
    pub ws_callback: Arc<Mutex<HashMap<u64, Sender<String>>>>,
}

impl Context {
    pub fn new(sender: WSSender) -> Self {
        Self {
            ws_sender: sender,
            ws_callback: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_callback(&self, id: u64, callback: Sender<String>) {
        let mut ws_callback = self.ws_callback.lock();
        ws_callback.insert(id, callback);
    }

    pub fn remove_callback(&self, id: u64) {
        let mut ws_callback = self.ws_callback.lock();
        ws_callback.remove(&id);
    }
}

pub enum CallError {
    InternalWS(WSError),
    InternalRecv(RecvError),
    InternalSerde(SerdeError),
    InternalSync(String),
    Response(JSONRPCError),
    Timeout(RecvTimeoutError),
}

impl From<WSError> for CallError {
    fn from(error: WSError) -> Self {
        CallError::InternalWS(error)
    }
}

impl From<RecvError> for CallError {
    fn from(error: RecvError) -> Self {
        CallError::InternalRecv(error)
    }
}

impl From<SerdeError> for CallError {
    fn from(error: SerdeError) -> Self {
        CallError::InternalSerde(error)
    }
}

impl<T> From<PoisonError<T>> for CallError {
    fn from(error: PoisonError<T>) -> Self {
        CallError::InternalSync(format!("{:?}", error))
    }
}

impl From<JSONRPCError> for CallError {
    fn from(error: JSONRPCError) -> Self {
        CallError::Response(error)
    }
}

impl From<RecvTimeoutError> for CallError {
    fn from(error: RecvTimeoutError) -> Self {
        CallError::Timeout(error)
    }
}

pub fn call_no_arg<Res>(context: Context, method: &str) -> Result<Res, CallError>
where
    Res: DeserializeOwned, {
    call_one_arg(context, method, Value::Null)
}

pub fn call_one_arg<Arg, Res>(context: Context, method: &str, arg: Arg) -> Result<Res, CallError>
where
    Arg: Serialize,
    Res: DeserializeOwned, {
    call_many_args(context, method, vec![arg])
}

pub fn call_many_args<Arg, Res>(context: Context, method: &str, args: Arg) -> Result<Res, CallError>
where
    Arg: Serialize,
    Res: DeserializeOwned, {
    let (tx, rx) = channel();
    let args_value = serde_json::to_value(args)?;
    let id = rand::random();
    let request = MethodCall {
        jsonrpc: Some(Version::V2),
        method: method.to_string(),
        params: Some(Params::Array(args_value.as_array().expect("This should be an array").clone())),
        id: Id::Num(id),
    };
    let serialized_request = serde_json::to_string(&request)?;
    context.add_callback(id, tx);
    ctrace!("send JSONRPC {}", serialized_request);
    context.ws_sender.send(Message::Text(serialized_request))?;
    let receive_result = rx.recv_timeout(Duration::new(10, 0));
    context.remove_callback(id);
    let received_string = receive_result?;
    ctrace!("Receive JSONRPC {}", received_string);

    let res = serde_json::from_str(&received_string)?;

    match res {
        Output::Success(success) => {
            let result = serde_json::from_value(success.result)?;
            Ok(result)
        }
        Output::Failure(failure) => Err(failure.error.into()),
    }
}

pub fn serialize_notification<Arg>(method: &str, arg: Arg) -> String
where
    Arg: Serialize, {
    let arg_value = serde_json::to_value(arg).expect("Should success serialization");
    let arg_object = arg_value.as_object().unwrap();
    let noti = Notification {
        jsonrpc: Some(Version::V2),
        method: method.to_string(),
        params: Some(Params::Map(arg_object.clone())),
    };
    serde_json::to_string(&noti).expect("Should success serialize")
}

// Called on websocket thread
pub fn on_receive(context: Context, text: String) {
    match on_receive_internal(context, text) {
        Ok(_) => {}
        Err(err) => cerror!("{}", err),
    }
}

fn on_receive_internal(context: Context, text: String) -> Result<(), String> {
    let json_parsed_result: Output = serde_json::from_str(&text)
        .map_err(|err| format!("Cannot parse response from agent, data is {}\n{}", text.clone(), err))?;

    let id = json_parsed_result.id();
    let id = match id {
        Id::Null => Err(id),
        Id::Str(_) => Err(id),
        Id::Num(id) => Ok(id),
    }
    .map_err(|id| format!("Invalid id {:#?}", id))?;

    let mut ws_callback = context.ws_callback.lock();
    let result = {
        let callback = ws_callback.get_mut(&id).ok_or_else(|| format!("Invalid id {}", id))?;
        callback
            .send(text.clone())
            .map_err(|err| format!("Callback call failed, response was {}\n{}", text.clone(), err))
    };
    ws_callback.remove(&id);
    result
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CallError::InternalWS(err) => write!(f, "Call Internal Error {}", err),
            CallError::InternalRecv(err) => write!(f, "Call Internal Error {}", err),
            CallError::InternalSerde(err) => write!(f, "Call Internal Error {}", err),
            CallError::InternalSync(err) => write!(f, "Call Internal Error {}", err),
            CallError::Response(err) => write!(f, "JSONRPC error {:?}", err),
            CallError::Timeout(err) => write!(f, "Timeout {}", err),
        }
    }
}

impl fmt::Debug for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for CallError {}
