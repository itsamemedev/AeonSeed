use aeoncode::AeonCodePlugin;
use ai::AiPlugin;
use bevy::prelude::*;
use game::GamePlugin;
use localization::LocalizationPlugin;
use render::RenderPlugin;
use seednet::SeedNetPlugin;

fn main() {
    // Initialize Bevy application with basic plugins
    App::new()
        .add_plugins((
            DefaultPlugins,
            SeedNetPlugin,
            AiPlugin,
            AeonCodePlugin,
            GamePlugin,
            RenderPlugin,
            LocalizationPlugin,
        ))
        .run();
}

mod aeoncode;
mod ai;
mod game;
mod localization;
mod render;
mod seednet;
mod zones;
