use std::time::Duration;

use bevy::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cluster::{core::ClusterCore, node::ClusterStatus};

/// Messages broadcast to seeds across the network.
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum SeedBroadcast {
    MergeRequest(Uuid, Uuid),
    ResourceWarning(Uuid),
    GlobalEvent(String),
}

/// Timer controlling how often cluster status is evaluated.
#[derive(Resource)]
pub struct ClusterMonitorTimer(pub Timer);

impl Default for ClusterMonitorTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1800), TimerMode::Repeating))
    }
}

/// AI plugin overseeing all seed nodes.
pub struct ClusterAiPlugin;

impl Plugin for ClusterAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SeedBroadcast>();
        app.init_resource::<ClusterMonitorTimer>();
        app.add_systems(Update, monitor_cluster);
    }
}

/// Periodically inspect nodes and emit global events.
fn monitor_cluster(
    time: Res<Time>,
    mut timer: ResMut<ClusterMonitorTimer>,
    mut cluster: ResMut<ClusterCore>,
    mut broadcasts: EventWriter<SeedBroadcast>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let entropy_threshold = cluster.entropy_threshold;
    let resonance_threshold = cluster.resonance_threshold;

    let mut to_archive = Vec::new();

    for (id, node) in cluster.nodes.iter_mut() {
        if node.entropy_index > entropy_threshold {
            to_archive.push(*id);
            node.status = ClusterStatus::Archived;
            broadcasts.send(SeedBroadcast::GlobalEvent(format!(
                "Seed {} archived due to high entropy",
                id
            )));
        } else if node.resonance_index > resonance_threshold {
            broadcasts.send(SeedBroadcast::GlobalEvent(format!(
                "Seed {} reached high resonance",
                id
            )));
        }
        node.last_updated = Utc::now();
    }

    for id in to_archive {
        cluster.archived_seeds.push(id);
    }
}
