pub mod api;

pub struct ModdingPlugin;

impl bevy::prelude::Plugin for ModdingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((api::ModdingApiPlugin,));
    }
}
