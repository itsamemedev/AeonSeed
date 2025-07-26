use bevy::prelude::*;

/// Marker component for traveling clans without a home seed.
#[derive(Component)]
pub struct NomadClan;

pub struct NomadClanPlugin;

impl Plugin for NomadClanPlugin {
    fn build(&self, _app: &mut App) {
        // Systems for roaming clan logic will go here.
    }
}
