use bevy::prelude::*;
use serde::Deserialize;
use std::fs;
use aeonseed_engine::EnginePlugin;

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
    let _ = dotenvy::dotenv();
    let _config = load_config();
    let headless = std::env::args().any(|a| a == "--no-render");
    let mut app = App::new();
    if headless {
        app.add_plugins(MinimalPlugins);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.add_plugins(EnginePlugin).run();
}
