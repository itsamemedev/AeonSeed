use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct NetworkSim {
    pub latency: Duration,
    pub packet_loss: f32,
}

pub struct NetworkSimPlugin;

fn simulate_network(time: Res<Time>, mut timer: Local<Timer>, settings: Res<NetworkSim>) {
    if timer.tick(time.delta()).just_finished() {
        if rand::random::<f32>() < settings.packet_loss {
            // drop packet
        }
    }
}

impl Plugin for NetworkSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkSim>()
            .add_systems(Update, simulate_network);
    }
}
