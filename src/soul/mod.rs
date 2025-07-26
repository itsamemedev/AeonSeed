pub mod modifications;
pub mod reincarnation;

pub struct SoulExtPlugin;

impl bevy::prelude::Plugin for SoulExtPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((modifications::SoulModPlugin, reincarnation::ReincarnationPlugin));
    }
}
