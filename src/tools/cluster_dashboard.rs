use bevy::prelude::*;

/// Simple resource for web dashboard state.
#[derive(Resource, Default)]
pub struct DashboardState {
    pub enabled: bool,
}

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DashboardState>();
    }
}
