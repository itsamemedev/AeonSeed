use bevy::prelude::*;

/// Event emitted when a party clears a dungeon.
#[derive(Event)]
pub struct DungeonCompleted {
    pub dungeon: String,
    pub party: Vec<Entity>,
}

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DungeonCompleted>();
    }
}
