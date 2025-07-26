pub mod morphing;
pub mod shadow_classes;

pub struct ClassExtPlugin;

impl bevy::prelude::Plugin for ClassExtPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((morphing::ClassMorphPlugin, shadow_classes::ShadowClassPlugin));
    }
}
