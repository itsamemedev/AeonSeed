use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Reputation {
    pub standing: i32,
}

pub struct ReputationPlugin;

impl Plugin for ReputationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Reputation>();
    }
}
