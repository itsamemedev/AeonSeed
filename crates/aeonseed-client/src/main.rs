use bevy::prelude::*;
use aeonseed_engine::EnginePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnginePlugin)
        .run();
}
