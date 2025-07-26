use bevy::prelude::*;

/// Arena mixing PvP and PvE influenced by AI.
#[derive(Component)]
pub struct PvpveArena;

pub struct PvpvePlugin;

impl Plugin for PvpvePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArenaOutcome>();
    }
}

/// Result of a PvPvE arena match.
#[derive(Event)]
pub struct ArenaOutcome {
    pub arena: Entity,
    pub victorious_side: String,
}
