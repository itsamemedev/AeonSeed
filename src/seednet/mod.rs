use bevy::prelude::*;

pub mod dna;
pub mod seed;
pub mod health;
pub mod network;

/// Root plugin combining all SeedNet systems.
pub struct SeedNetPlugin;

impl Plugin for SeedNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dna::DnaPlugin,
            seed::SeedPlugin,
            health::SeedHealthPlugin,
            network::SeedNetApiPlugin,
        ));
    }
}
