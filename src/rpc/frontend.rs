use super::router::Router;

pub fn add_routing(routing_table: &mut Router) {
    let f: fn() -> String = ping;
    routing_table.add_route("ping", Box::new(f));
    routing_table.add_route("ping", Box::new(ping as fn() -> String));
    routing_table.add_route("add1", Box::new(add1 as fn(i32) -> i32));
}

fn ping() -> String {
    "pong".to_string()
}

fn add1(x: i32) -> i32 {
    x + 1
}
