use bevy::prelude::*;

/// Event triggered when players start a duel.
#[derive(Event)]
pub struct DuelStarted {
    pub challenger: Entity,
    pub opponent: Entity,
}

pub struct PvpPlugin;

impl Plugin for PvpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DuelStarted>();
    }
}
