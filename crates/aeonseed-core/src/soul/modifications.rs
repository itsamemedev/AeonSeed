use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Reflect)]
pub struct SoulModification {
    pub courage_bonus: f32,
    pub resonance_bonus: f32,
}

#[derive(Resource, Default)]
pub struct SoulMods {
    pub mods: Vec<SoulModification>,
}

pub struct SoulModPlugin;

impl Plugin for SoulModPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoulMods>();
        app.register_type::<SoulModification>();
    }
}
