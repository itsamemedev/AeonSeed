use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MemoryState {
    pub active_quests: Vec<String>,
}

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MemoryState>();
    }
}
