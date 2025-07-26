use bevy::prelude::*;

pub mod redis;
pub mod updater;

pub struct InfraPlugin;

impl Plugin for InfraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<redis::RedisConn>();
    }
}
