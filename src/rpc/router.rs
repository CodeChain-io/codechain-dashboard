use std::collections::HashMap;
use std::sync::Arc;

use serde::de::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;

use super::super::types::HandlerContext;

pub trait Route {
    fn run(&self, context: Arc<HandlerContext>, value: Value) -> Option<Value>;
}

pub struct Router {
    table: HashMap<&'static str, Box<Route>>,
}

impl<Arg, Result> Route for fn(Arc<HandlerContext>, Arg) -> Result
where
    Result: Serialize,
    for<'de> Arg: Deserialize<'de>,
{
    fn run(&self, context: Arc<HandlerContext>, value: Value) -> Option<Value> {
        let arg = serde_json::from_value(value).expect("Should be fixed");
        let result = self(context, arg);
        let value_result = serde_json::to_value(result).expect("SHould be fixed");
        Some(value_result)
    }
}

impl<Result> Route for fn(Arc<HandlerContext>) -> Result
where
    Result: Serialize,
{
    fn run(&self, context: Arc<HandlerContext>, _value: Value) -> Option<Value> {
        let result = self(context);
        let value_result = serde_json::to_value(result).expect("SHould be fixed");
        Some(value_result)
    }
}

pub enum Error {
    MethodNotFound,
}

impl Router {
    pub fn new() -> Self {
        let table: HashMap<&'static str, Box<Route>> = HashMap::new();
        Self {
            table,
        }
    }

    pub fn add_route(&mut self, method: &'static str, route: Box<Route>) {
        self.table.insert(method, route);
    }

    pub fn run(&self, context: Arc<HandlerContext>, method: &str, arg: Value) -> Result<Option<Value>, Error> {
        let route = self.table.get(method);
        match route {
            None => Err(Error::MethodNotFound),
            Some(route) => Ok(route.run(context, arg)),
        }
    }
}
