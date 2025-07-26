use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalEventState {
    pub active_events: Vec<String>,
}

pub struct DynamicEventsPlugin;

impl Plugin for DynamicEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalEventState>();
    }
}
