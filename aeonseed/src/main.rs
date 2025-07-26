use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod startup;
mod seed;
mod ui;
mod player;

use startup::StartupPlugin;
use seed::SeedPlugin;
use ui::UiPlugin;
use player::PlayerPlugin;

fn main() {
    dotenvy::dotenv().ok();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(SeedPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
