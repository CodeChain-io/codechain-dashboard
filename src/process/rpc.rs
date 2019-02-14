use jsonrpc_core;
use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub enum CallRPCError {
    Web(reqwest::Error),
    Format(String),
}

impl ::std::error::Error for CallRPCError {}

impl ::std::fmt::Display for CallRPCError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            CallRPCError::Web(err) => write!(f, "Web request error while sending RPC to CodeChain: {}", err),
            CallRPCError::Format(err) => write!(f, "Cannot parse CodeChain's response as JSON: {}", err),
        }
    }
}

pub struct RPCClient {
    rpc_port: u16,
    http_client: reqwest::Client,
}

impl RPCClient {
    pub fn new(rpc_port: u16) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            rpc_port,
            http_client,
        }
    }

    /// Return JSONRPC response object
    /// Example: {"jsonrpc": "2.0", "result": 19, "id": 1}
    pub fn call_rpc(&self, method: String, arguments: Vec<Value>) -> Result<Value, CallRPCError> {
        let params = jsonrpc_core::Params::Array(arguments);

        let jsonrpc_request = jsonrpc_core::MethodCall {
            jsonrpc: None,
            method,
            params: Some(params),
            id: jsonrpc_core::Id::Num(1),
        };

        ctrace!(PROCESS, "Send JSONRPC to CodeChain {:#?}", jsonrpc_request);

        let url = format!("http://127.0.0.1:{}/", self.rpc_port);
        let mut response = self.http_client.post(&url).json(&jsonrpc_request).send().map_err(CallRPCError::Web)?;

        let response: jsonrpc_core::Response = response.json().map_err(|err| CallRPCError::Format(err.to_string()))?;

        ctrace!(PROCESS, "Recieve JSONRPC response from CodeChain {:#?}", response);
        let value = serde_json::to_value(response).expect("Should success jsonrpc type to Value");

        Ok(value)
    }
}
