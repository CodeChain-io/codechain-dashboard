use super::super::common_rpc_types::NodeName;
use super::types::{ClientExtra, ClientQueryResult};

pub enum Event {
    ClientUpdated {
        before: Box<Option<ClientQueryResult>>,
        after: Box<ClientQueryResult>,
    },
    ConnectionChanged {
        added: Vec<(NodeName, NodeName)>,
        removed: Vec<(NodeName, NodeName)>,
    },
    ClientExtraUpdated {
        name: NodeName,
        before: Option<ClientExtra>,
        after: ClientExtra,
    },
}

pub trait EventSubscriber: Send {
    fn on_event(&self, event: Event);
}
