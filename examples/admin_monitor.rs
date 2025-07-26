use aeonseed_core::admin::AdminPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AdminPlugin)
        .run();
}
