use bevy::prelude::*;

/// Event emitted when a world ritual succeeds.
#[derive(Event)]
pub struct RitualCompleted {
    pub ritual: String,
}

pub struct RitualPlugin;

impl Plugin for RitualPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RitualCompleted>();
    }
}
