#[macro_use]
extern crate log;
extern crate iron;

extern crate codechain_rpc as crpc;
extern crate jsonrpc_core;
extern crate primitives as cprimitives;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

#[macro_use]
mod logger;
mod agent;
mod common_rpc_types;
mod frontend;
mod jsonrpc;
mod router;
mod rpc;

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use iron::prelude::*;
use iron::status;
use ws::listen;

use self::agent::agent::SendAgentRPC;
use self::agent::handler::WebSocketHandler as AgentHandler;
use self::agent::service::{Service as AgentService, ServiceSender as AgentServiceSender};
use self::frontend::api::add_routing as add_frontend_routing;
use self::frontend::handler::WebSocketHandler as FrontendHandler;
use self::frontend::types::Context as FrontendContext;
use self::logger::init as logger_init;
use self::router::Router;

fn main() {
    logger_init().expect("Logger should be initialized");

    let agent_service_sender = AgentService::run_thread();
    let agent_service_for_frontend = agent_service_sender.clone();
    let web_handler = WebHandler::new(agent_service_sender.clone());

    let frontend_join = thread::Builder::new()
        .name("frontend listen".to_string())
        .spawn(move || {
            let count = Rc::new(Cell::new(0));
            let mut frontend_router = Arc::new(Router::new());
            add_frontend_routing(Arc::get_mut(&mut frontend_router).unwrap());
            let frontend_context = FrontendContext {
                agent_service: agent_service_for_frontend,
            };
            listen("127.0.0.1:3012", move |out| FrontendHandler {
                out,
                count: count.clone(),
                context: frontend_context.clone(),
                router: frontend_router.clone(),
            }).unwrap();
        })
        .expect("Should success listening frontend");

    let agent_join = thread::Builder::new()
        .name("agent listen".to_string())
        .spawn(move || {
            let count = Rc::new(Cell::new(0));
            listen("127.0.0.1:4012", |out| AgentHandler::new(out, count.clone(), agent_service_sender.clone()))
                .unwrap();
        })
        .expect("Should success listening agent");

    let webserver_join = thread::Builder::new()
        .name("webserver".to_string())
        .spawn(move || {
            let _server = Iron::new(web_handler).http("localhost:5012").unwrap();
            cinfo!("Webserver listening on 5012");
        })
        .expect("Should success open webserver");

    frontend_join.join().expect("Join frotend listner");
    agent_join.join().expect("Join agent listner");
    webserver_join.join().expect("Join webserver");
}

struct WebHandler {
    agent_service_sender: Mutex<AgentServiceSender>,
}

impl WebHandler {
    fn new(agent_service_sender: AgentServiceSender) -> Self {
        Self {
            agent_service_sender: Mutex::new(agent_service_sender),
        }
    }
}

impl iron::Handler for WebHandler {
    fn handle(&self, req: &mut iron::Request) -> IronResult<iron::Response> {
        let paths = req.url.path();
        if paths.len() != 2 {
            cwarn!("Invalid web request {}", req.url);
            return Ok(Response::with(status::NotFound))
        }

        if paths.get(0).expect("Already checked") != &"log" {
            cwarn!("Invalid web request {}", req.url);
            return Ok(Response::with(status::NotFound))
        }

        let node_address = paths.get(1).expect("Already checked");
        let node_address = node_address
            .parse()
            .map_err(|_| iron::IronError::new(WebError::new("Invalid socket address"), status::BadRequest))?;
        ctrace!("Get log for agent-{}", node_address);

        let agent = self
            .agent_service_sender
            .lock()
            .expect("Should success get lock")
            .get_agent(node_address)
            .ok_or_else(|| iron::IronError::new(WebError::new("Not Found"), status::NotFound))?;

        let log =
            agent.shell_get_codechain_log().map_err(|err| iron::IronError::new(err, status::InternalServerError))?;

        use iron::mime;
        let content_type = "text/plain".parse::<mime::Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, log)))
    }
}

#[derive(Debug)]
struct WebError {
    value: String,
}

impl WebError {
    fn new(s: &str) -> Self {
        WebError {
            value: s.to_string(),
        }
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for WebError {}
