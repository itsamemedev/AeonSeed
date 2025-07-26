use bevy::prelude::*;

/// Arena mixing PvP and PvE influenced by AI.
#[derive(Component)]
pub struct PvpveArena;

pub struct PvpvePlugin;

impl Plugin for PvpvePlugin {
    fn build(&self, _app: &mut App) {
        // Systems orchestrating dynamic arenas will go here.
    }
}
