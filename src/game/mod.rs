use bevy::prelude::*;

pub mod classes;
pub mod player;
pub mod professions;
pub mod quests;
pub mod soul;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            classes::ClassesPlugin,
            professions::ProfessionsPlugin,
            quests::QuestPlugin,
            soul::SoulPlugin,
        ));
    }
}
