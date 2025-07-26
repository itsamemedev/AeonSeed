use aeonseed_core::mentor::{MentorPairs, MentorPlugin};
use bevy::prelude::*;

#[test]
fn mentor_plugin_adds_resource() {
    let mut app = App::new();
    app.add_plugins(MentorPlugin);
    app.update();
    assert!(app.world.contains_resource::<MentorPairs>());
}
