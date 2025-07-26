use bevy::prelude::*;

/// Resource describing scarcity levels which influence NPC behavior and economy.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct ResourcePressure {
    /// 0.0 abundant - 1.0 extreme shortage
    pub scarcity_index: f32,
}

pub struct ResourcePressurePlugin;

impl Plugin for ResourcePressurePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResourcePressure>();
        app.register_type::<ResourcePressure>();
    }
}
