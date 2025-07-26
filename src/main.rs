use bevy::prelude::*;
use aeonseed::{
    aeoncode::AeonCodePlugin,
    ai::AiPlugin,
    cluster::ClusterPlugin,
    game::GamePlugin,
    localization::LocalizationPlugin,
    render::RenderPlugin,
    seednet::SeedNetPlugin,
    account::AccountPlugin,
    ui::UiPlugin,
    events::EventsPlugin,
    infra::InfraPlugin,
};

fn main() {
    // Initialize Bevy application with basic plugins
    App::new()
        .add_plugins((
            DefaultPlugins,
            SeedNetPlugin,
            ClusterPlugin,
            AiPlugin,
            AeonCodePlugin,
            GamePlugin,
            RenderPlugin,
            LocalizationPlugin,
            AccountPlugin,
            UiPlugin,
            EventsPlugin,
            InfraPlugin,
        ))
        .run();
}

