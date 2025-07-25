use bevy::prelude::*;
use bevy_core_pipeline::core_3d::Camera3dBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
