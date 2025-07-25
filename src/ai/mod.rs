use bevy::prelude::*;

pub mod language;
pub mod memory;
pub mod npc_core;
pub mod world_ai;

/// Root plugin bundling all AI systems.
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world_ai::WorldAiPlugin,
            npc_core::NpcPlugin,
            memory::MemoryPlugin,
            language::LanguagePlugin,
        ));
    }
}
