use super::types::AgentState;

pub enum Event {
    AgentUpdated {
        before: Option<AgentState>,
        after: AgentState,
    },
}

pub trait EventSubscriber: Send {
    fn on_event(&self, event: Event);
}
