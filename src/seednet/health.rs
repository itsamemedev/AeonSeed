use super::SeedEntity;
use bevy::prelude::*;

/// Plugin monitoring Seed health.
pub struct SeedHealthPlugin;

impl Plugin for SeedHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, monitor_seed_health);
    }
}

/// Placeholder system for checking Seed status.
fn monitor_seed_health(query: Query<&SeedEntity>) {
    for seed in &query {
        let _ = seed.id; // Replace with real health checks
    }
}
