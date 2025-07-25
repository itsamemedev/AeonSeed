use bevy::prelude::*;

pub mod core;
pub mod node;
pub mod topology;

use core::ClusterCore;

/// Plugin initializing the global cluster management resource.
pub struct ClusterPlugin;

impl Plugin for ClusterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClusterCore>();
    }
}
