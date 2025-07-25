use bevy::prelude::*;
use uuid::Uuid;

use super::seed::Seed;

/// Plugin exposing basic SeedNet networking stubs.
pub struct SeedNetApiPlugin;

impl Plugin for SeedNetApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (find_seed_stub, reactivate_seed_stub));
    }
}

/// Example system that would locate seeds over the network.
fn find_seed_stub(query: Query<&Seed>) {
    for seed in &query {
        let _ = seed.id; // placeholder for network discovery
    }
}

/// Example system that reactivates inactive seeds.
fn reactivate_seed_stub(mut query: Query<&mut Seed>) {
    for mut seed in &mut query {
        if !seed.is_active {
            seed.is_active = true;
        }
    }
}

/// Utility used by other modules to search for a seed locally.
pub fn find_seed_by_id(query: &Query<&Seed>, id: Uuid) -> Option<Seed> {
    for seed in query.iter() {
        if seed.id == id {
            return Some(seed.clone());
        }
    }
    None
}
