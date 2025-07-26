use bevy::prelude::*;

/// Event triggered when a player reincarnates with a new perspective.
#[derive(Event)]
pub struct Reincarnate {
    pub player: Entity,
}

pub struct ReincarnationPlugin;

impl Plugin for ReincarnationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Reincarnate>();
    }
}
