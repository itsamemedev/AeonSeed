use bevy::prelude::*;

/// Resource controlling AI-based music generation.
#[derive(Resource, Default)]
pub struct MoodEngineState {
    pub last_mood: String,
}

pub struct MoodEnginePlugin;

impl Plugin for MoodEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoodEngineState>();
    }
}
