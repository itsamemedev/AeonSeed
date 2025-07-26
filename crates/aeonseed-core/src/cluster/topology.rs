use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Styles of connection between seed nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyKind {
    Ring,
    Star,
    Mesh,
}

/// Representation of seed connectivity in the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterTopology {
    pub kind: TopologyKind,
    /// Adjacency list describing connections.
    pub connections: HashMap<Uuid, Vec<Uuid>>,
}

impl Default for ClusterTopology {
    fn default() -> Self {
        Self {
            kind: TopologyKind::Mesh,
            connections: HashMap::new(),
        }
    }
}
