use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::node::ClusterNode;
use super::topology::ClusterTopology;

/// Central resource managing all nodes and global thresholds.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCore {
    pub nodes: HashMap<Uuid, ClusterNode>,
    pub topology: ClusterTopology,
    pub archived_seeds: Vec<Uuid>,
    pub entropy_threshold: f32,
    pub resonance_threshold: f32,
}

impl Default for ClusterCore {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
            topology: ClusterTopology::default(),
            archived_seeds: Vec::new(),
            entropy_threshold: 0.8,
            resonance_threshold: 0.8,
        }
    }
}
