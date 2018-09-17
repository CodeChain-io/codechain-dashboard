#[derive(Debug, Serialize, Deserialize)]
pub enum NodeStatus {
    Run,
    Stop,
    Error,
    UFO,
}
