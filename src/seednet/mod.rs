use bevy::prelude::*;

pub mod discovery;
pub mod dna;
pub mod health;

/// Root plugin combining all SeedNet systems.
pub struct SeedNetPlugin;

impl Plugin for SeedNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            discovery::SeedDiscoveryPlugin,
            health::SeedHealthPlugin,
            dna::DnaPlugin,
        ));
    }
}

/// Represents a handle to a dynamically loaded Seed instance.
#[derive(Component)]
pub struct SeedEntity {
    pub id: u64,
}
