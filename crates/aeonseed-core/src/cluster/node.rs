use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::ethics::WorldAlignment;

/// Operational state of a seed within the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterStatus {
    /// Active and participating in the global network.
    Active,
    /// Temporarily disabled or unreachable.
    Inactive,
    /// Archived and no longer updated.
    Archived,
}

/// Metadata about a single seed node managed by the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNode {
    pub seed_id: Uuid,
    pub status: ClusterStatus,
    pub force_alignment: WorldAlignment,
    pub entropy_index: f32,
    pub resonance_index: f32,
    pub player_activity: u32,
    pub last_updated: DateTime<Utc>,
}

impl Default for ClusterNode {
    fn default() -> Self {
        Self {
            seed_id: Uuid::nil(),
            status: ClusterStatus::Inactive,
            force_alignment: WorldAlignment::default(),
            entropy_index: 0.0,
            resonance_index: 0.0,
            player_activity: 0,
            last_updated: Utc::now(),
        }
    }
}
