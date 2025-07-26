use bevy::prelude::*;

/// Marker component for UI screenshot button.
#[derive(Component)]
pub struct FeedbackButton;

pub struct FeedbackPlugin;

impl Plugin for FeedbackPlugin {
    fn build(&self, _app: &mut App) {
        // Implementation for screenshot and log upload would go here.
    }
}
