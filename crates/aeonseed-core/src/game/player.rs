use bevy::prelude::*;

use super::soul::SoulProfile;
use super::ethics::ForceAffinity;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, SoulProfile::default(), ForceAffinity::default()));
}
