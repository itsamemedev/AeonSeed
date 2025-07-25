use bevy::prelude::*;
use std::collections::HashMap;

/// Represents persistent memory for NPCs keyed by Entity id.
#[derive(Resource, Default)]
pub struct NpcMemory {
    pub memories: HashMap<Entity, Vec<String>>,
}

/// Plugin handling NPC memory graph.
pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NpcMemory>();
    }
}
