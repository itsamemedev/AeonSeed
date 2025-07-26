use aeonseed_core::admin::AdminPlugin;
use bevy::prelude::*;

#[test]
fn admin_plugin_builds() {
    let mut app = App::new();
    app.add_plugins(AdminPlugin);
    app.update();
}
