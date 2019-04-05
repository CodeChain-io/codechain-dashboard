use std::net::SocketAddr;

use jsonrpc_core::types::{Failure, Output, Success};
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use super::super::common_rpc_types::{BlackList, BlockId, NodeStatus, PendingParcel, StructuredLog, WhiteList};
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
        self.call_rpc(status, "net_getEstablishedPeers", Vec::new())
    }

    pub fn get_best_block_id(&self, status: NodeStatus) -> Result<Option<BlockId>, String> {
        let response: Option<ChainGetBestBlockIdResponse> =
            self.call_rpc(status, "chain_getBestBlockId", Vec::new())?;

        Ok(response.map(|response| BlockId {
            block_number: response.number,
            hash: response.hash,
        }))
    }

    pub fn version(&self, status: NodeStatus) -> Result<Option<String>, String> {
        self.call_rpc(status, "version", Vec::new())
    }

    pub fn commit_hash(&self, status: NodeStatus) -> Result<Option<String>, String> {
        self.call_rpc(status, "commitHash", Vec::new())
    }

    pub fn get_pending_parcels(&self, _status: NodeStatus) -> Result<Vec<PendingParcel>, String> {
        //        self.call_rpc(status, "chain_getPendingParcels")
        Ok(Vec::new())
    }

    pub fn get_whitelist(&self, status: NodeStatus) -> Result<Option<WhiteList>, String> {
        self.call_rpc(status, "net_getWhitelist", Vec::new())
    }

    pub fn get_blacklist(&self, status: NodeStatus) -> Result<Option<BlackList>, String> {
        self.call_rpc(status, "net_getBlacklist", Vec::new())
    }

    pub fn get_logs(&self, status: NodeStatus) -> Result<Vec<StructuredLog>, String> {
        if status != NodeStatus::Run {
            return Ok(Default::default())
        }
        let response = self.sender.shell_get_codechain_log().map_err(|err| format!("{}", err))?;

        Ok(response)
    }

    fn call_rpc<T>(&self, status: NodeStatus, method: &str, params: Vec<Value>) -> Result<T, String>
    where
        T: Default + DeserializeOwned, {
        if status != NodeStatus::Run {
            return Ok(Default::default())
        }

        let response =
            self.sender.codechain_call_rpc((method.to_string(), params)).map_err(|err| format!("{}", err))?;

        let response: T = match response {
            Output::Success(Success {
                result,
                ..
            }) => serde_json::from_value(result).map_err(|err| format!("{}", err))?,
            Output::Failure(Failure {
                error,
                ..
            }) => return Err(format!("{} error {:#?}", method, error)),
        };

        Ok(response)
    }
}
