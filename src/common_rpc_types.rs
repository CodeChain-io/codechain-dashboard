#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum NodeStatus {
    Run,
    Stop,
    Error,
    UFO,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}
