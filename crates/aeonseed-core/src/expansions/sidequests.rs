use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SidequestState {
    pub generated: u32,
}

pub struct SidequestPlugin;

impl Plugin for SidequestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SidequestState>();
    }
}
