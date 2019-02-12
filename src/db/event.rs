use super::super::common_rpc_types::NodeName;
use super::types::{AgentExtra, AgentQueryResult};

pub enum Event {
    AgentUpdated {
        before: Box<Option<AgentQueryResult>>,
        after: Box<AgentQueryResult>,
    },
    ConnectionChanged {
        added: Vec<(NodeName, NodeName)>,
        removed: Vec<(NodeName, NodeName)>,
    },
    AgentExtraUpdated {
        name: NodeName,
        before: Option<AgentExtra>,
        after: AgentExtra,
    },
}

pub trait EventSubscriber: Send {
    fn on_event(&self, event: Event);
}
