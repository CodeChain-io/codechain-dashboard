use ws;
use ws::{CloseCode, Error as WSError, Handler, Handshake, Result, Sender as WSSender};

use super::super::client;
use super::super::jsonrpc;

pub struct WebSocketHandler {
    pub out: WSSender,
    pub count: usize,
    pub client_service: client::ServiceSender,
    pub jsonrpc_context: jsonrpc::Context,
}

impl WebSocketHandler {
    pub fn new(out: WSSender, client_service: client::ServiceSender) -> Self {
        let jsonrpc_context = jsonrpc::Context::new(out.clone());
        client_service
            .send(client::Message::InitializeClient(jsonrpc_context.clone()))
            .expect("Should success send InitializeClient to service");
        Self {
            out,
            count: 0,
            client_service,
            jsonrpc_context,
        }
    }
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count += 1;
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        // Tell the user the current count
        ctrace!("The number of live connections is {}", self.count);

        match msg {
            ws::Message::Text(text) => jsonrpc::on_receive(self.jsonrpc_context.clone(), text),
            _ => {
                cwarn!("Byte data received from client");
            }
        };
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => cinfo!("The client is done with the connection."),
            CloseCode::Away => cinfo!("The client is leaving the site."),
            CloseCode::Abnormal => cinfo!("Closing handshake failed! Unable to obtain closing status from client."),
            _ => cinfo!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count -= 1;
    }

    fn on_error(&mut self, err: WSError) {
        cerror!("The server encountered an error: {:?}", err);
    }
}
