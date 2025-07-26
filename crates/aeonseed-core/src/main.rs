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
    world::WorldPlugin,
    instance::InstancePlugin,
    lang::LangPlugin,
    class::ClassExtPlugin,
    soul::SoulExtPlugin,
    skills::SkillsPlugin,
    clan::ClanPlugin,
    tools::ToolsPlugin,
    modding::ModdingPlugin,
    audio::mood_engine::MoodEnginePlugin,
    reporting::feedback::FeedbackPlugin,
};

fn main() {
    // Initialize Bevy application with basic plugins
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
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
        .add_plugins((
            WorldPlugin,
            InstancePlugin,
            LangPlugin,
            ClassExtPlugin,
            SoulExtPlugin,
            SkillsPlugin,
            ClanPlugin,
            ToolsPlugin,
            ModdingPlugin,
            MoodEnginePlugin,
            FeedbackPlugin,
        ))
        .run();
}

