pub mod climate;
pub mod resource_pressure;
pub mod anomalies;

use climate::ClimatePlugin;
use resource_pressure::ResourcePressurePlugin;
use anomalies::AnomalyPlugin;

/// Aggregates all world-level systems.
pub struct WorldPlugin;

impl bevy::prelude::Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ClimatePlugin, ResourcePressurePlugin, AnomalyPlugin));
    }
}
