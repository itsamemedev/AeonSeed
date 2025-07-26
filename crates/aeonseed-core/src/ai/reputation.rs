//! KI-gestuetztes Rufsystem zwischen Spielern und NPCs.
//! Noch nicht implementiert.
use bevy::prelude::*;

/// Speichert den aktuellen Rufwert.
#[derive(Resource, Default)]
pub struct ReputationScore {
    pub value: i32,
}

/// Plugin zur Integration in die AI-Pipeline.
pub struct ReputationAiPlugin;

impl Plugin for ReputationAiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReputationScore>();
        // TODO: Anbindung an NPC-Persoenlichkeit
    }
}
