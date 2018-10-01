use std::net::SocketAddr;

use jsonrpc_core::types::{Failure, Output, Success};
use serde_json;

use super::super::common_rpc_types::{BlockId, NodeStatus};
use super::agent::{AgentSender, SendAgentRPC};
use super::types::ChainGetBestBlockIdResponse;

pub struct CodeChainRPC {
    sender: AgentSender,
}

impl CodeChainRPC {
    pub fn new(sender: AgentSender) -> Self {
        Self {
            sender,
        }
    }

    pub fn get_peers(&self, status: NodeStatus) -> Result<Vec<SocketAddr>, String> {
        if status != NodeStatus::Run {
            return Ok(Vec::new())
        }

        let response = self
            .sender
            .codechain_call_rpc(("net_getEstablishedPeers".to_string(), Vec::new()))
            .map_err(|err| format!("{}", err))?;

        let peers: Vec<SocketAddr> = match response {
            Output::Success(Success {
                result,
                ..
            }) => serde_json::from_value(result).map_err(|err| format!("{}", err))?,
            Output::Failure(Failure {
                error,
                ..
            }) => return Err(format!("get_peers error {:#?}", error)),
        };

        Ok(peers)
    }

    pub fn get_best_block_id(&self, status: NodeStatus) -> Result<Option<BlockId>, String> {
        if status != NodeStatus::Run {
            return Ok(None)
        }

        let response = self
            .sender
            .codechain_call_rpc(("chain_getBestBlockId".to_string(), Vec::new()))
            .map_err(|err| format!("{}", err))?;

        let best_block_id: ChainGetBestBlockIdResponse = match response {
            Output::Success(Success {
                result,
                ..
            }) => serde_json::from_value(result).map_err(|err| format!("{}", err))?,
            Output::Failure(Failure {
                error,
                ..
            }) => return Err(format!("get_peers error {:#?}", error)),
        };

        Ok(Some(BlockId {
            block_number: best_block_id.number,
            hash: best_block_id.hash,
        }))
    }
}
