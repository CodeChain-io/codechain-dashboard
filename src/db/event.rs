use super::super::common_rpc_types::NodeName;
use super::types::AgentState;

pub enum Event {
    AgentUpdated {
        before: Option<AgentState>,
        after: AgentState,
    },
    ConnectionChanged {
        added: Vec<(NodeName, NodeName)>,
        removed: Vec<(NodeName, NodeName)>,
    },
}

pub trait EventSubscriber: Send {
    fn on_event(&self, event: Event);
}
