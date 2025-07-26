use bevy::prelude::*;

pub mod dungeon;
pub mod raid;
pub mod pvp;
pub mod pvpve;
pub mod ritual;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dungeon::DungeonPlugin,
            raid::RaidPlugin,
            pvp::PvpPlugin,
            pvpve::PvpvePlugin,
            ritual::RitualPlugin,
        ));
    }
}
