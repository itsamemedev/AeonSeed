use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub name: String,
    pub class: String,
    pub profession: String,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player);
    }
}

fn create_player(mut commands: Commands) {
    commands.spawn(Player {
        name: "Held".into(),
        class: "Wanderer".into(),
        profession: "Farmer".into(),
    });
}
