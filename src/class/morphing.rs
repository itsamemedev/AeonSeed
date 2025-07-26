use bevy::prelude::*;

/// Ritual-based class switching story events.
#[derive(Event)]
pub struct ClassMorphEvent {
    pub player: Entity,
    pub new_class: String,
}

pub struct ClassMorphPlugin;

impl Plugin for ClassMorphPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClassMorphEvent>();
    }
}
