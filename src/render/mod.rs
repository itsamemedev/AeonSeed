use bevy::prelude::*;

pub mod camera;
pub mod models;
pub mod zones;

/// Rendering plugin focused on low-end devices.
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin,));
    }
}
