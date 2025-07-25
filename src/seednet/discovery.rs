use super::SeedEntity;
use bevy::prelude::*;

/// Plugin responsible for discovering and loading Seeds.
pub struct SeedDiscoveryPlugin;

impl Plugin for SeedDiscoveryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (discover_seeds, unload_inactive_seeds));
    }
}

/// Example system that simulates discovering new Seeds.
fn discover_seeds(mut commands: Commands, query: Query<Entity, With<SeedEntity>>) {
    // If no seed exists, spawn a placeholder
    if query.is_empty() {
        commands.spawn(SeedEntity { id: 0 });
    }
}

/// Example system that simulates unloading Seeds.
fn unload_inactive_seeds(mut commands: Commands, query: Query<(Entity, &SeedEntity)>) {
    for (entity, seed) in &query {
        if seed.id != 0 {
            // Placeholder for unload logic
            commands.entity(entity).despawn_recursive();
        }
    }
}
