use std::collections::HashMap;
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use cprimitives::H256;
use serde_json;

pub type NodeName = String;

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

pub type ShellUpdateCodeChainRequest = (ShellStartCodeChainRequest, UpdateCodeChainRequest);

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
    pub binary_checksum: String,
}

pub type PendingTransaction = serde_json::Value;

pub type Tag = String;

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteList {
    pub list: Vec<(IpAddr, Tag)>,
    pub enabled: bool,
}

pub type BlackList = WhiteList;

pub type NetworkUsage = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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
    pub disk_usages: Vec<HardwareUsage>,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum UpdateCodeChainRequest {
    #[serde(rename_all = "camelCase")]
    Git {
        commit_hash: String,
    },
    #[serde(rename_all = "camelCase")]
    Binary {
        #[serde(rename = "binaryURL")]
        binary_url: String,
        binary_checksum: String,
    },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GraphPeriod {
    Minutes5,
    Hour,
    Day,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphCommonArgs {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub period: GraphPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNetworkOutAllRow {
    pub node_name: String,
    pub time: DateTime<Utc>,
    pub value: f32,
}

pub type GraphNetworkOutAllAVGRow = GraphNetworkOutAllRow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNetworkOutNodeExtensionRow {
    pub extension: String,
    pub time: DateTime<Utc>,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNetworkOutNodePeerRow {
    pub peer: String,
    pub time: DateTime<Utc>,
    pub value: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_day() {
        let period = GraphPeriod::Day;
        assert_eq!("\"day\"", &serde_json::to_string(&period).unwrap());
    }
    #[test]
    fn serialize_minutes5() {
        let period = GraphPeriod::Minutes5;
        assert_eq!("\"minutes5\"", &serde_json::to_string(&period).unwrap());
    }
    #[test]
    fn serialize_hour() {
        let period = GraphPeriod::Hour;
        assert_eq!("\"hour\"", &serde_json::to_string(&period).unwrap());
    }
}
