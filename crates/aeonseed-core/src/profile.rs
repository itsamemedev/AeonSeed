use bevy::prelude::*;
use tracing_tracy::TracyLayer;
use tracing_subscriber::prelude::*;

pub fn init_tracy() {
    let tracy_layer = TracyLayer::new();
    tracing_subscriber::registry().with(tracy_layer).init();
}
