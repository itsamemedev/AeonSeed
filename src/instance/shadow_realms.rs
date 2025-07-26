use bevy::prelude::*;

/// Parallel seeds with inverted ethics and logic.
#[derive(Resource, Default)]
pub struct ShadowRealmRegistry {
    pub active_realms: Vec<String>,
}

pub struct ShadowRealmsPlugin;

impl Plugin for ShadowRealmsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShadowRealmRegistry>();
    }
}
