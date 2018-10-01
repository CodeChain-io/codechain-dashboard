use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

use super::super::common_rpc_types::{BlockId, NodeName, NodeStatus, NodeVersion};

#[derive(PartialEq, Clone, Debug, Default)]
pub struct AgentState {
    pub name: NodeName,
    pub status: NodeStatus,
    pub address: Option<SocketAddr>,
    pub peers: Vec<SocketAddr>,
    pub best_block_id: Option<BlockId>,
    pub version: Option<NodeVersion>,
}

pub type Connection = (SocketAddr, SocketAddr);

pub struct Connections {
    data: HashSet<Connection>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
        }
    }

    pub fn update(&mut self, before: &AgentState, after: &AgentState) -> (Vec<Connection>, Vec<Connection>) {
        if before.address.is_none() || after.address.is_none() {
            return (Vec::new(), Vec::new())
        }

        let added = Self::get_added(before, after);
        let removed = Self::get_removed(before, after);

        let mut ret_added = Vec::new();
        let mut ret_removed = Vec::new();

        for added_element in added {
            if self.data.insert(added_element.clone()) {
                ret_added.push(added_element);
            }
        }

        for removed_element in removed {
            if self.data.remove(&removed_element) {
                ret_removed.push(removed_element);
            }
        }

        (ret_added, ret_removed)
    }

    fn get_added(before: &AgentState, after: &AgentState) -> Vec<Connection> {
        let before_peers: HashSet<&SocketAddr> = before.peers.iter().collect();
        after
            .peers
            .iter()
            .filter(|peer| !before_peers.contains(*peer))
            .map(|peer| Self::make_tuple(after.address.unwrap(), *peer))
            .collect()
    }

    fn get_removed(before: &AgentState, after: &AgentState) -> Vec<Connection> {
        let after_peers: HashSet<&SocketAddr> = after.peers.iter().collect();
        before
            .peers
            .iter()
            .filter(|peer| !after_peers.contains(*peer))
            .map(|peer| Self::make_tuple(after.address.unwrap(), *peer))
            .collect()
    }

    fn make_tuple(a: SocketAddr, b: SocketAddr) -> Connection {
        let mut default_hasher = DefaultHasher::new();
        a.hash(&mut default_hasher);
        let a_hash = default_hasher.finish();

        let mut default_hasher = DefaultHasher::new();
        b.hash(&mut default_hasher);
        let b_hash = default_hasher.finish();

        if a_hash < b_hash {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn get_all(&self) -> Vec<Connection> {
        self.data.iter().map(|connection| *connection).collect()
    }
}
