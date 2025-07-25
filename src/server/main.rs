use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use aeonseed::{
    aeoncode::AeonCodePlugin,
    ai::AiPlugin,
    cluster::ClusterPlugin,
    game::GamePlugin,
    localization::LocalizationPlugin,
    seednet::SeedNetPlugin,
};

#[derive(Deserialize, Default)]
struct ServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

fn load_config() -> ServerConfig {
    fs::read_to_string("config.yaml")
        .ok()
        .and_then(|c| serde_yaml::from_str(&c).ok())
        .unwrap_or_default()
}

fn main() {
    // Lade Umgebungsvariablen
    let _ = dotenvy::dotenv();
    let _config = load_config();
    App::new()
        .add_plugins((
            MinimalPlugins,
            SeedNetPlugin,
            ClusterPlugin,
            AiPlugin,
            AeonCodePlugin,
            GamePlugin,
            LocalizationPlugin,
        ))
        .run();
}
