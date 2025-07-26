use aeonseed_core::dynamic_ui::DynamicUiPlugin;
use bevy::prelude::*;

#[test]
fn plugin_registers() {
    let mut app = App::new();
    app.add_plugins(DynamicUiPlugin);
    app.update();
}
