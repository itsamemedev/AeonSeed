use bevy::prelude::*;
use chrono::{DateTime, Utc};

/// Different anomaly effects that can warp time flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeAnomalyKind {
    Accelerate,
    Reverse,
}

/// Active anomaly affecting the world.
#[derive(Debug, Clone)]
pub struct TimeAnomaly {
    pub kind: TimeAnomalyKind,
    pub ends_at: DateTime<Utc>,
}

#[derive(Resource, Default)]
pub struct AnomalyRegistry {
    pub active: Vec<TimeAnomaly>,
}

pub struct AnomalyPlugin;

impl Plugin for AnomalyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnomalyRegistry>();
    }
}
