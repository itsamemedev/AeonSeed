use bevy::prelude::*;

/// Hidden questline exclusive classes.
#[derive(Event)]
pub struct DiscoverShadowClass {
    pub player: Entity,
    pub class_name: String,
}

pub struct ShadowClassPlugin;

impl Plugin for ShadowClassPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DiscoverShadowClass>();
    }
}
