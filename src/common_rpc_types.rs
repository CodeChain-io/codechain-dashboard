use std::net::IpAddr;

use cprimitives::H256;
use serde_json;

pub type NodeName = String;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum NodeStatus {
    Starting,
    Run,
    Stop,
    Error,
    UFO,
}

impl Default for NodeStatus {
    fn default() -> NodeStatus {
        NodeStatus::Stop
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}

pub type Connection = (NodeName, NodeName);

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockId {
    pub block_number: i64,
    pub hash: H256,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeVersion {
    pub version: String,
    pub hash: String,
}

pub type PendingParcel = serde_json::Value;

pub type Tag = String;

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteList {
    pub list: Vec<(IpAddr, Tag)>,
    pub enabled: bool,
}

pub type BlackList = WhiteList;
