use std::collections::HashMap;

use serde::de::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;

use super::types::{RPCError, RPCResult};

pub trait Route {
    fn run(&self, value: Value) -> RPCResult<Value>;
}

pub struct Router {
    table: HashMap<&'static str, Box<Route>>,
}

impl<Arg, Result> Route for fn(Arg) -> RPCResult<Result>
where
    Result: Serialize,
    for<'de> Arg: Deserialize<'de>,
{
    fn run(&self, value: Value) -> RPCResult<Value> {
        let arg = serde_json::from_value(value)?;
        let result = self(arg)?;
        if let Some(result) = result {
            Ok(Some(serde_json::to_value(result)?))
        } else {
            Ok(None)
        }
    }
}

impl<Result> Route for fn() -> RPCResult<Result>
where
    Result: Serialize,
{
    fn run(&self, _value: Value) -> RPCResult<Value> {
        let result = self()?;
        if let Some(result) = result {
            let value_result = serde_json::to_value(result)?;
            Ok(Some(value_result))
        } else {
            Ok(None)
        }
    }
}

pub enum Error {
    MethodNotFound,
    RPC(RPCError),
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

    pub fn run(&self, method: &str, arg: Value) -> Result<Option<Value>, Error> {
        let route = self.table.get(method);
        match route {
            None => Err(Error::MethodNotFound),
            Some(route) => match route.run(arg) {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::RPC(err)),
            },
        }
    }
}
