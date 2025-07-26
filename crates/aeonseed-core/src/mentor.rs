use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MentorPairs(pub Vec<(u64, u64)>);

pub struct MentorPlugin;

impl Plugin for MentorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MentorPairs>();
    }
}
