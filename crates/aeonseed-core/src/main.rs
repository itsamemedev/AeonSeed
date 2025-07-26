use aeonseed::{
    account::AccountPlugin, aeoncode::AeonCodePlugin, ai::AiPlugin,
    audio::mood_engine::MoodEnginePlugin, clan::ClanPlugin, class::ClassExtPlugin,
    cluster::ClusterPlugin, events::EventsPlugin, expansions::ExpansionsPlugin, game::GamePlugin,
    infra::InfraPlugin, instance::InstancePlugin, lang::LangPlugin,
    localization::LocalizationPlugin, modding::ModdingPlugin, render::RenderPlugin,
    reporting::feedback::FeedbackPlugin, seednet::SeedNetPlugin, skills::SkillsPlugin,
    soul::SoulExtPlugin, tools::ToolsPlugin, ui::UiPlugin, world::WorldPlugin,
};
use bevy::prelude::*;

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
            ExpansionsPlugin,
            MoodEnginePlugin,
            FeedbackPlugin,
        ))
        .run();
}
