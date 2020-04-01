use jsonrpc_core::types::Version;
use parking_lot::Mutex;
use serde_json::{Error as SerdeError, Value};
use std::io::Error as IoError;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{write_all, AsyncRead};
use tokio::prelude::future::Future;
use tokio::prelude::stream::Stream;
use tokio::runtime::Runtime;
use tokio_codec::{FramedRead, LinesCodec};
use tokio_uds::UnixStream;

#[derive(Debug)]
pub enum CallRPCError {
    Serde(SerdeError),
    Io(IoError),
    NoResponse,
}

impl ::std::fmt::Display for CallRPCError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        match self {
            CallRPCError::Serde(err) => err.fmt(f),
            CallRPCError::Io(err) => err.fmt(f),
            CallRPCError::NoResponse => write!(f, "CodeChain doesn't respond"),
        }
    }
}

pub struct RPCClient {
    path: String,
}

impl RPCClient {
    pub fn new(path: String) -> Self {
        Self {
            path,
        }
    }

    /// Return JSONRPC response object
    /// Example: {"jsonrpc": "2.0", "result": 19, "id": 1}
    pub fn call_rpc(&self, method: String, arguments: Vec<Value>) -> Result<Value, CallRPCError> {
        let jsonrpc_request = jsonrpc_core::MethodCall {
            jsonrpc: Some(Version::V2),
            method,
            params: Some(jsonrpc_core::Params::Array(arguments)),
            id: jsonrpc_core::Id::Num(1),
        };

        ctrace!(PROCESS, "Send JSONRPC to CodeChain {:#?}", jsonrpc_request);

        if !Path::new(&self.path).exists() {
            cerror!(PROCESS, "IPC file does not exist, please check CodeChain's config whether ipc is disabled or not");
        }

        let mut rt = Runtime::new()?;

        let response = Arc::new(Mutex::new(None));
        let last_error = Arc::new(Mutex::new(None));

        let body = serde_json::to_vec(&jsonrpc_request)?;
        let cloned_response = Arc::clone(&response);
        let cloned_last_error = Arc::clone(&last_error);
        let stream = UnixStream::connect(&self.path).map_err(CallRPCError::from).and_then(|stream| {
            let (read, write) = stream.split();
            let framed_read = FramedRead::new(read, LinesCodec::new());

            write_all(write, body).map_err(CallRPCError::from).and_then(move |_| {
                framed_read
                    .map_err(CallRPCError::from)
                    .filter_map(move |s| match serde_json::from_str(&s) {
                        Ok(json) => Some(json),
                        Err(err) => {
                            *cloned_last_error.lock() = Some(err);
                            None
                        }
                    })
                    .take(1)
                    .for_each(move |response| {
                        *cloned_response.lock() = Some(response);
                        Ok(())
                    })
            })
        });

        // TODO: Remove the below thread blocking code.
        rt.block_on(stream)?;

        let mut response = response.lock();
        let mut last_error = last_error.lock();

        if let Some(result) = response.take() {
            ctrace!(PROCESS, "Receive JSONRPC response from CodeChain {:#?}", result);
            Ok(result)
        } else if let Some(err) = last_error.take() {
            Err(err.into())
        } else {
            Err(CallRPCError::NoResponse)
        }
    }
}

impl From<SerdeError> for CallRPCError {
    fn from(err: SerdeError) -> Self {
        CallRPCError::Serde(err)
    }
}

impl From<IoError> for CallRPCError {
    fn from(err: IoError) -> Self {
        CallRPCError::Io(err)
    }
}
