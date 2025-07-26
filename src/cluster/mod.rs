use bevy::prelude::*;

pub mod core;
pub mod node;
pub mod topology;
pub mod magnetism;
pub mod camouflage;

use core::ClusterCore;

/// Plugin initializing the global cluster management resource.
pub struct ClusterPlugin;

impl Plugin for ClusterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClusterCore>();
        app.add_plugins((magnetism::MagnetismPlugin, camouflage::CamouflagePlugin));
    }
}
