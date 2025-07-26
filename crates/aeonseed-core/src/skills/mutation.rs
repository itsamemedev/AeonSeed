use bevy::prelude::*;

/// Mutation of skills depending on seed instability or player choice.
#[derive(Event)]
pub struct SkillMutate {
    pub player: Entity,
    pub skill_name: String,
}

pub struct SkillMutationPlugin;

impl Plugin for SkillMutationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkillMutate>();
    }
}
