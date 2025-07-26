use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DecisionProgress {
    pub choices: Vec<String>,
}

pub struct DecisionProgressPlugin;

impl Plugin for DecisionProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DecisionProgress>();
    }
}
