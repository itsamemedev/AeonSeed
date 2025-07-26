use bevy::prelude::*;

/// Event for chaotic cross-seed collisions.
#[derive(Event)]
pub struct SeedCollision {
    pub seed_a: Entity,
    pub seed_b: Entity,
}

pub struct MagnetismPlugin;

impl Plugin for MagnetismPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SeedCollision>();
    }
}
