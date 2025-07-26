use bevy::prelude::*;

/// Zones providing buffs or debuffs when controlled by clans.
#[derive(Component)]
pub struct FortressZone;

pub struct FortressPlugin;

impl Plugin for FortressPlugin {
    fn build(&self, _app: &mut App) {
        // Systems for clan fortress control will go here.
    }
}
