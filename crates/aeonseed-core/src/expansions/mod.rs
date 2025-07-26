use bevy::prelude::*;

pub mod benchmarking;
pub mod currency;
pub mod decisions;
pub mod devtools;
pub mod dynamic_events;
pub mod evolving_zones;
pub mod feedback;
pub mod legacy;
pub mod memory;
pub mod mentoring;
pub mod regional_rules;
pub mod reputation;
pub mod scripting;
pub mod seed_fusion;
pub mod sidequests;
pub mod snapshots;
pub mod streaming;
pub mod timed_events;
pub mod twitch;
pub mod voting;
pub mod web_companion;

/// Aggregates all expansion plugins.
pub struct ExpansionsPlugin;

impl Plugin for ExpansionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dynamic_events::DynamicEventsPlugin,
            evolving_zones::EvolvingZonesPlugin,
            currency::CurrencyPlugin,
            regional_rules::RegionalRulesPlugin,
            timed_events::TimedEventsPlugin,
            decisions::DecisionProgressPlugin,
            memory::MemoryPlugin,
            sidequests::SidequestPlugin,
            reputation::ReputationPlugin,
            legacy::LegacyPlugin,
            devtools::DevToolsPlugin,
            scripting::ScriptingPlugin,
            streaming::StreamingPlugin,
            snapshots::SnapshotPlugin,
            benchmarking::BenchmarkingPlugin,
            voting::VotingPlugin,
            mentoring::MentoringPlugin,
            twitch::TwitchPlugin,
            web_companion::WebCompanionPlugin,
            feedback::MlFeedbackPlugin,
            seed_fusion::SeedFusionPlugin,
        ));
    }
}
