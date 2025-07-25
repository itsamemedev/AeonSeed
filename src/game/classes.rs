use bevy::prelude::*;

#[derive(Component)]
pub struct Class;

pub struct ClassesPlugin;

impl Plugin for ClassesPlugin {
    fn build(&self, _app: &mut App) {
        // Dynamic class system via world interaction
    }
}
