use std::net::IpAddr;

use cprimitives::H256;
use serde_json;

pub type NodeName = String;
pub type CommitHash = String;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum NodeStatus {
    Starting,
    Run,
    Stop,
    Updating,
    Error,
    UFO,
}

impl Default for NodeStatus {
    fn default() -> NodeStatus {
        NodeStatus::Stop
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellUpdateCodeChainRequest {
    pub env: String,
    pub args: String,
    pub commit_hash: String,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructuredLog {
    pub level: String,
    pub target: String,
    pub message: String,
    pub timestamp: String,
    pub thread_name: String,
}
