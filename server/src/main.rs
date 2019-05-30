#[macro_use]
extern crate log;

extern crate chrono;
extern crate jsonrpc_core;
#[macro_use]
extern crate lazy_static;
extern crate parking_lot;
extern crate postgres;
extern crate primitives as cprimitives;
extern crate rand;
extern crate regex;
extern crate sendgrid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate slack_hook;
extern crate ws;

#[macro_use]
mod logger;
mod agent;
mod common_rpc_types;
mod daily_reporter;
mod db;
mod event_propagator;
mod frontend;
mod jsonrpc;
mod noti;
mod router;
mod rpc;
mod util;

use std::sync::Arc;
use std::thread;

use ws::listen;

use self::event_propagator::EventPropagator;
use self::logger::init as logger_init;
use self::noti::NotiBuilder;
use self::router::Router;

fn main() {
    logger_init().expect("Logger should be initialized");

    let mut noti_builder = NotiBuilder::default();
    if let Ok(slack_hook_url) = std::env::var("SLACK_WEBHOOK_URL") {
        cinfo!("Set slack");
        noti_builder.slack(slack_hook_url);
    }
    match (std::env::var("SENDGRID_API_KEY"), std::env::var("SENDGRID_TO")) {
        (Ok(api_key), Ok(to)) => {
            cinfo!("Set email to {}", to);
            noti_builder.sendgrid(api_key, to);
        }
        (Ok(_), _) => {
            panic!("You set a sendgrid api key, but not a destination");
        }
        (_, Ok(_)) => {
            panic!("You set a sendgrid destination, but not an api key");
        }
        _ => {}
    }
    let noti = noti_builder.build();

    // FIXME: move to config
    let db_user = "codechain-agent-hub";
    let db_password = "preempt-entreat-bell-chanson";

    let frontend_service_sender = frontend::Service::run_thread();
    let event_propagator = Box::new(EventPropagator::new(frontend_service_sender.clone()));
    let db_service_sender = db::Service::run_thread(db::ServiceNewArg {
        event_subscriber: event_propagator,
        db_user: db_user.to_string(),
        db_password: db_password.to_string(),
    });
    let agent_service_sender = agent::Service::run_thread(db_service_sender.clone(), Arc::clone(&noti));
    let agent_service_for_frontend = agent_service_sender.clone();

    let db_service_sender_for_frontend = db_service_sender.clone();
    let frontend_join = thread::Builder::new()
        .name("frontend listen".to_string())
        .spawn(move || {
            let mut frontend_router = Arc::new(Router::new());
            frontend::add_routing(Arc::get_mut(&mut frontend_router).unwrap());
            let frontend_context = frontend::Context {
                agent_service: agent_service_for_frontend,
                db_service: db_service_sender_for_frontend.clone(),
                passphrase: std::env::var("PASSPHRASE").unwrap_or_else(|_| "passphrase".to_string()),
            };
            listen("0.0.0.0:3012", move |out| frontend::WebSocketHandler {
                out,
                context: frontend_context.clone(),
                router: Arc::clone(&frontend_router),
                frontend_service: frontend_service_sender.clone(),
            })
            .unwrap();
        })
        .expect("Should success listening frontend");

    let agent_service_for_agent = agent_service_sender.clone();
    let agent_join = thread::Builder::new()
        .name("agent listen".to_string())
        .spawn(move || {
            listen("0.0.0.0:4012", |out| agent::WebSocketHandler::new(out, agent_service_for_agent.clone())).unwrap();
        })
        .expect("Should success listening agent");

    let daily_reporter_join = daily_reporter::start(noti, db_service_sender, agent_service_sender);

    frontend_join.join().expect("Join frontend listener");
    agent_join.join().expect("Join agent listener");
    daily_reporter_join.join().expect("Join daily reporter");
}
