use super::rpc::{RPCError, RPCResponse};
use serde::de::Deserialize;
use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;

pub trait Route {
    type Context;
    fn run(&self, context: Self::Context, value: Value) -> RPCResponse<Value>;
}

pub struct Router<C> {
    table: HashMap<&'static str, Box<dyn Route<Context = C>>>,
}

impl<Arg, Result, C> Route for fn(context: C, Arg) -> RPCResponse<Result>
where
    Result: Serialize,
    for<'de> Arg: Deserialize<'de>,
{
    type Context = C;
    fn run(&self, context: Self::Context, value: Value) -> RPCResponse<Value> {
        let arg = serde_json::from_value(value)?;
        let result = self(context, arg)?;
        if let Some(result) = result {
            Ok(Some(serde_json::to_value(result)?))
        } else {
            Ok(None)
        }
    }
}

impl<Result, C> Route for fn(context: C) -> RPCResponse<Result>
where
    Result: Serialize,
{
    type Context = C;
    fn run(&self, context: Self::Context, _value: Value) -> RPCResponse<Value> {
        let result = self(context)?;
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

impl<C> Router<C> {
    pub fn new() -> Self {
        let table: HashMap<&'static str, Box<dyn Route<Context = C>>> = HashMap::new();
        Self {
            table,
        }
    }

    pub fn add_route(&mut self, method: &'static str, route: Box<dyn Route<Context = C>>) {
        self.table.insert(method, route);
    }

    pub fn run(&self, context: C, method: &str, arg: Value) -> Result<Option<Value>, Error> {
        let route = self.table.get(method);
        match route {
            None => Err(Error::MethodNotFound),
            Some(route) => match route.run(context, arg) {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::RPC(err)),
            },
        }
    }
}
