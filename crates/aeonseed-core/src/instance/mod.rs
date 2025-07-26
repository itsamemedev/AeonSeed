pub mod shadow_realms;

use shadow_realms::ShadowRealmsPlugin;

pub struct InstancePlugin;

impl bevy::prelude::Plugin for InstancePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ShadowRealmsPlugin,));
    }
}
