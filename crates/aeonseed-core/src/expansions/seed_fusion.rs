use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SeedFusionState {
    pub pending: bool,
}

pub struct SeedFusionPlugin;

impl Plugin for SeedFusionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SeedFusionState>();
    }
}
