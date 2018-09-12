use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use jsonrpc_core::types::{Call, Error as JSONRPCError, ErrorCode, Failure, Id, MethodCall, Response, Success};
use serde_json;
use serde_json::Value;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Message, Result, Sender};

use super::rpc::router::Router;

pub struct WebSocketHandler {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
    pub routing_table: Arc<Router>,
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        println!("The number of live connections is {}", self.count.get());

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
                        ..
                    })) => {
                        // FIXME
                        let ret = match self.routing_table.run(&method, Value::from(0)) {
                            Ok(Some(value)) => value,
                            _ => Value::from("not routed"),
                        };
                        Some(
                            Success {
                                jsonrpc: None,
                                result: ret,
                                id,
                            }.into(),
                        )
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
            // Echo the message back
            self.out.send(Message::Text(serialized))
        } else {
            Ok(())
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!("Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: WSError) {
        println!("The server encountered an error: {:?}", err);
    }
}
