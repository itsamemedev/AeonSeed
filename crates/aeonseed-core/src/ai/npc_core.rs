use bevy::prelude::*;

/// Core NPC behaviors plugin.
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_think);
    }
}

/// Basic NPC thinking logic driving idle animations.
fn npc_think() {
    // A real implementation would evaluate goals and memories here.
}
