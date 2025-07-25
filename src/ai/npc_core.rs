use bevy::prelude::*;

/// Core NPC behaviors plugin.
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_think);
    }
}

/// Basic NPC thinking logic placeholder.
fn npc_think() {
    // NPC AI placeholder
}
