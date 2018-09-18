#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum NodeStatus {
    Run,
    Stop,
    Error,
    UFO,
}
