use super::super::common_rpc_types::NodeName;
use super::types::AgentQueryResult;

pub enum Event {
    AgentUpdated {
        before: Option<AgentQueryResult>,
        after: AgentQueryResult,
    },
    ConnectionChanged {
        added: Vec<(NodeName, NodeName)>,
        removed: Vec<(NodeName, NodeName)>,
    },
}

pub trait EventSubscriber: Send {
    fn on_event(&self, event: Event);
}
