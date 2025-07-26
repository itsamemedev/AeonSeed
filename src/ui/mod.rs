use bevy::prelude::*;

pub mod auth_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, auth_ui::auth_ui_system);
    }
}
