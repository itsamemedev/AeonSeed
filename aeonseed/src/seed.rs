use bevy::prelude::*;
use mongodb::sync::Client as MongoClient;
use redis::Client as RedisClient;
use std::{fs::{OpenOptions, create_dir_all}, io::Write};

pub struct SeedPlugin;

#[derive(Resource)]
pub struct SeedInfo {
    pub id: String,
}

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(init_seed())
            .add_systems(Startup, log_seed_info);
    }
}

pub fn init_seed() -> SeedInfo {
    let id = std::env::var("SEED_ID").unwrap_or_else(|_| "local-seed".into());
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let redis_uri = std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    create_dir_all("logs").ok();
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/db_connections.log")
        .ok();

    let mongo_res = MongoClient::with_uri_str(&mongo_uri);
    let redis_res = RedisClient::open(redis_uri);

    if let Some(mut file) = log.as_mut() {
        let _ = writeln!(file, "MongoDB connection: {}", if mongo_res.is_ok() { "Ok" } else { "Error" });
        let _ = writeln!(file, "Redis connection: {}", if redis_res.is_ok() { "Ok" } else { "Error" });
    }

    let _mongo = mongo_res.ok();
    let _redis = redis_res.ok();
    SeedInfo { id }
}

fn log_seed_info(info: Res<SeedInfo>) {
    info!("Seed {} aktiviert", info.id);
}
