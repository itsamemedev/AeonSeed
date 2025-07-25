use bevy::prelude::*;
use super::memory::NpcMemoryBank;

/// Global struct representing the world's overarching AI.
#[derive(Resource, Default)]
pub struct WorldConsciousness;

/// Generates and updates world events and quests.
pub struct WorldAiPlugin;

impl Plugin for WorldAiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConsciousness>();
        app.add_systems(Update, world_consciousness_update);
    }
}
fn world_consciousness_update(
    memories: Res<NpcMemoryBank>,
    mut _world: ResMut<WorldConsciousness>,
) {
    // Iterate over NPC memories and derive global events or quests.
    for (_entity, npc_mem) in memories.memories.iter() {
        let _ = npc_mem.short_term.len();
        // More complex processing will be implemented here.
    }
}
