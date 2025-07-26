use redis::Client;
use bevy::prelude::*;
#[derive(Resource)]

pub struct RedisConn {
    pub client: Client,
}

impl Default for RedisConn {
    fn default() -> Self {
        let client = Client::open("redis://127.0.0.1/").unwrap();
        Self { client }
    }
}
