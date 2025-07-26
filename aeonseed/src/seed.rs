use bevy::prelude::*;
use mongodb::sync::Client as MongoClient;
use redis::Client as RedisClient;

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

fn init_seed() -> SeedInfo {
    let id = std::env::var("SEED_ID").unwrap_or_else(|_| "local-seed".into());
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let redis_uri = std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let _mongo = MongoClient::with_uri_str(&mongo_uri).ok();
    let _redis = RedisClient::open(redis_uri).ok();
    SeedInfo { id }
}

fn log_seed_info(info: Res<SeedInfo>) {
    info!("Seed {} aktiviert", info.id);
}
