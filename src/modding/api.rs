use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomClassConfig {
    pub name: String,
}

#[derive(Resource, Default)]
pub struct ModdingRegistry {
    pub classes: Vec<CustomClassConfig>,
}

pub struct ModdingApiPlugin;

impl Plugin for ModdingApiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModdingRegistry>();
    }
}
