use bevy::prelude::*;
use crate::aeoncode::{matrix::AeonCodeMatrix, spawn_fragments_for_seeds};

/// AI plugin controlling appearance and tracking of Aeon Code fragments.
pub struct AeonCodeAiPlugin;

impl Plugin for AeonCodeAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fragments_for_seeds);
        app.add_systems(Update, monitor_global_progress);
    }
}

/// Example system reacting to reconstructed level.
fn monitor_global_progress(matrix: Res<AeonCodeMatrix>) {
    if matrix.reconstructed_level >= 1.0 {
        // Placeholder for world-changing event once the code is solved.
        info!("The Aeon Code has been fully deciphered!");
    }
}
