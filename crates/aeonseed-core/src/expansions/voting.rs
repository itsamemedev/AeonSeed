use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct VotingState {
    pub proposals: Vec<String>,
}

pub struct VotingPlugin;

impl Plugin for VotingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VotingState>();
    }
}
