use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::seednet::seed::Seed;

/// Runtime statistics about a Seed instance.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SeedHealth {
    pub ram_usage_mb: f32,
    pub tick_rate: f32,
    pub player_count: u32,
    pub last_error: Option<String>,
    pub last_checked: DateTime<Utc>,
}

impl Default for SeedHealth {
    fn default() -> Self {
        Self {
            ram_usage_mb: 0.0,
            tick_rate: 0.0,
            player_count: 0,
            last_error: None,
            last_checked: Utc::now(),
        }
    }
}

/// Plugin that periodically checks seed health and archives inactive seeds.
pub struct SeedHealthPlugin;

impl Plugin for SeedHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (monitor_seed_health, archive_inactive_seeds));
    }
}

/// Dummy system updating health metrics.
fn monitor_seed_health(mut query: Query<&mut SeedHealth>) {
    for mut health in &mut query {
        health.ram_usage_mb = 100.0; // placeholder values
        health.tick_rate = 60.0;
        health.last_checked = Utc::now();
    }
}

/// Remove seeds that are no longer active and have no players.
fn archive_inactive_seeds(mut commands: Commands, query: Query<(Entity, &Seed, &SeedHealth)>) {
    for (entity, seed, health) in &query {
        if !seed.is_active && health.player_count == 0 {
            // Placeholder for archival logic
            commands.entity(entity).despawn_recursive();
        }
    }
}
