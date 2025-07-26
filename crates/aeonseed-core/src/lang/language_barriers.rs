use bevy::prelude::*;
use std::collections::HashMap;

/// Mapping of faction to known vocabulary progress.
#[derive(Resource, Default)]
pub struct LanguageKnowledge {
    pub progress: HashMap<String, f32>,
}

pub struct LanguageBarrierPlugin;

impl Plugin for LanguageBarrierPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LanguageKnowledge>();
    }
}
