use bevy::prelude::*;

#[derive(Component)]
pub struct Profession;

pub struct ProfessionsPlugin;

impl Plugin for ProfessionsPlugin {
    fn build(&self, _app: &mut App) {
        // Profession mechanics will be added here
    }
}
