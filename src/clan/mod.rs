pub mod fortresses;
pub mod nomads;

pub struct ClanPlugin;

impl bevy::prelude::Plugin for ClanPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((fortresses::FortressPlugin, nomads::NomadClanPlugin));
    }
}
