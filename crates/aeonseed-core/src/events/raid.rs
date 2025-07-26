use bevy::prelude::*;

/// Event signaled when a raid encounter begins.
#[derive(Event)]
pub struct RaidStarted {
    pub raid: String,
}

pub struct RaidPlugin;

impl Plugin for RaidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RaidStarted>();
    }
}
