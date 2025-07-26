use bevy::prelude::*;

#[derive(Resource)]
pub struct AccessibilitySettings {
    pub colorblind_mode: bool,
    pub font_scale: f32,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self { colorblind_mode: false, font_scale: 1.0 }
    }
}

pub struct AccessibilityPlugin;

impl Plugin for AccessibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccessibilitySettings>();
    }
}
