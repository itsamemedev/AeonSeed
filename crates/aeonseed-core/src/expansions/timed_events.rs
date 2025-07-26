use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TimedEvents {
    pub next_midnight_dungeon: Option<f64>,
}

pub struct TimedEventsPlugin;

impl Plugin for TimedEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimedEvents>();
    }
}
