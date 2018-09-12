use super::router::Router;

pub fn add_routing(routing_table: &mut Router) {
    routing_table.add_route("ping", Box::new(ping as fn() -> String));
}

fn ping() -> String {
    "pong".to_string()
}
