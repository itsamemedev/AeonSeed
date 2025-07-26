//! Grundlegende Guild-Funktionen.
//! Spaeter koennen hier Rangsystem und Gildenquests implementiert werden.
use bevy::prelude::*;

/// Aktive Gildenliste.
#[derive(Resource, Default)]
pub struct GuildRegistry {
    pub names: Vec<String>, // TODO: Persistenz in MongoDB
}

/// Plugin zum Laden des Guild-Systems.
pub struct GuildsPlugin;

impl Plugin for GuildsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GuildRegistry>();
        // TODO: Befehle und Systeme anlegen
    }
}
