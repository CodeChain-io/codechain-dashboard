use std::cell::Cell;
use std::rc::Rc;

use ws::{CloseCode, Error as WSError, Handler, Handshake, Message, Result, Sender as WSSender};

use super::super::jsonrpc;
use super::service::{Message as AgentServiceMessage, ServiceSender as AgentServiceSender};

pub struct WebSocketHandler {
    pub out: WSSender,
    pub count: Rc<Cell<u32>>,
    pub agent_service: AgentServiceSender,
    pub jsonrpc_context: jsonrpc::Context,
}

impl WebSocketHandler {
    pub fn new(out: WSSender, count: Rc<Cell<u32>>, agent_service: AgentServiceSender) -> Self {
        let jsonrpc_context = jsonrpc::Context::new(out.clone());
        agent_service
            .send(AgentServiceMessage::InitializeAgent(jsonrpc_context.clone()))
            .expect("Should success send InitializeAgent to service");
        Self {
            out,
            count,
            agent_service,
            jsonrpc_context,
        }
    }
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        ctrace!("The number of live connections is {}", self.count.get());

        match msg {
            Message::Text(text) => jsonrpc::on_receive(self.jsonrpc_context.clone(), text),
            _ => {
                cwarn!("Byte data received from agent");
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
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: WSError) {
        cerror!("The server encountered an error: {:?}", err);
    }
}
