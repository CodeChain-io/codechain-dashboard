use std::error::Error;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::fmt;

use ws;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Result, Sender, ErrorKind};

use super::super::jsonrpc;
use super::super::router::Router;
use super::types::Context;

#[derive(Debug)]
struct CustomError {}

impl Error for CustomError {

}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error")
    }
}

pub struct WebSocketHandler {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
    pub context: Context,
    pub router: Arc<Router<Context>>,
    pub frontend_service: super::ServiceSender,
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        if format!("/{}", self.context.passphrase) != handshake.request.resource() {
            return Err(WSError::new(ErrorKind::Custom(Box::new(CustomError {})),
                                    "Authorization Error"));
        }

        self.frontend_service
            .send(super::Message::AddWS(self.out.clone()))
            .expect("Should success adding ws to frontend_service");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        let response: Option<String> = match msg {
            ws::Message::Text(text) => {
                cinfo!("Receive {}", text);
                jsonrpc::handle(|method, arg| self.router.run(self.context.clone(), &method, arg), text)
            }
            _ => Some(jsonrpc::invalid_format()),
        };

        cinfo!("Response {:?}", response);
        if let Some(response) = response {
            self.out.send(ws::Message::Text(response))
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
        self.frontend_service
            .send(super::Message::RemoveWS(self.out.clone()))
            .expect("Should success remove ws from frontend_service");
    }

    fn on_error(&mut self, err: WSError) {
        if let Err(error) = self.out.close_with_reason(CloseCode::Error, "Error") {
            cerror!("Fail to close connection {}", error);
        }
        cerror!("The server encountered an error: {:?}", err);
    }
}
