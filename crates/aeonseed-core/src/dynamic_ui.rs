use bevy::prelude::*;

pub struct DynamicUiPlugin;

fn resize_ui(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.resolution.set(window.resolution.physical_width(), window.resolution.physical_height());
    }
}

impl Plugin for DynamicUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resize_ui);
    }
}
