use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LegacyState {
    pub inherited_points: u32,
}

pub struct LegacyPlugin;

impl Plugin for LegacyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyState>();
    }
}
