use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use jsonrpc_core::types::{Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Response, Success};
use serde_json;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Message, Result, Sender};

use super::rpc::router::{Error as RouterError, Router};
use super::types::HandlerContext;

pub struct WebSocketHandler {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
    pub router: Arc<Router>,
    pub context: Arc<HandlerContext>,
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        ctrace!("The number of live connections is {}", self.count.get());

        let response: Option<Response> = match msg {
            Message::Text(text) => {
                let deserialized = serde_json::from_str(&text);
                match deserialized {
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
                        match self.router.run(self.context.clone(), &method, value_params) {
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
                }
            }
            _ => Some(
                Failure {
                    jsonrpc: None,
                    id: Id::Null,
                    error: JSONRPCError::new(ErrorCode::ServerError(3)),
                }.into(),
            ),
        };

        if let Some(response) = response {
            let serialized = serde_json::to_string(&response).unwrap();
            self.out.send(Message::Text(serialized))
        } else {
            Ok(())
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => cinfo!("The client is done with the connection."),
            CloseCode::Away => cinfo!("The client is leaving the site."),
            CloseCode::Abnormal => cinfo!("Closing handshake failed! Unable to obtain closing status from client."),
            _ => cinfo!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: WSError) {
        cerror!("The server encountered an error: {:?}", err);
    }
}
