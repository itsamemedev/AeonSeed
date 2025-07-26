//! Seed-Fusion verbindet zwei bestehende Seeds zu einer neuen Welt.
//! Dies ist nur ein Stub und dient als Platzhalter fuer spaetere Implementierung.
use bevy::prelude::*;

/// Ressourcenhalter fuer anstehende Fusionsprozesse.
#[derive(Resource, Default)]
pub struct FusionQueue {
    pub pending: Vec<String>, // TODO: Seed-IDs speichern
}

/// Plugin zur Registrierung der Seed-Fusion.
pub struct SeedFusionPlugin;

impl Plugin for SeedFusionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FusionQueue>();
        // TODO: Systems zur Durchfuehrung der Fusion hinzufuegen
    }
}
