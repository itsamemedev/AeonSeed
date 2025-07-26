use bevy::prelude::*;
use aeonseed_core::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AeonCodePlugin,
            AiPlugin,
            ClusterPlugin,
            GamePlugin,
            LocalizationPlugin,
            SeedNetPlugin,
            AccountPlugin,
            UiPlugin,
            EventsPlugin,
            InfraPlugin,
        ));
    }
}
