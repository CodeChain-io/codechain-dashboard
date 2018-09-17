use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use ws::{CloseCode, Error as WSError, Handler, Handshake, Message, Result, Sender};

use super::super::jsonrpc;
use super::super::router::Router;

pub struct WebSocketHandler {
    pub out: Sender,
    pub count: Rc<Cell<u32>>,
    pub router: Arc<Router>,
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        ctrace!("The number of live connections is {}", self.count.get());

        let response: Option<String> = match msg {
            Message::Text(text) => jsonrpc::handle(|method, arg| {
                self.router.run(&method, arg)
            }, text),
            _ => Some(jsonrpc::invalid_format()),
        };

        if let Some(response) = response {
            self.out.send(Message::Text(response))
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
