use std::collections::HashMap;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::fragment::AeonFragment;

/// Central structure tracking global Aeon Code reconstruction progress.
#[derive(Debug, Default, Serialize, Deserialize, Resource)]
pub struct AeonCodeMatrix {
    /// All fragments currently known across seeds, keyed by fragment id.
    pub current_known: HashMap<Uuid, AeonFragment>,
    /// Percentage of how much of the code has been reconstructed.
    pub reconstructed_level: f32,
}

impl AeonCodeMatrix {
    /// Insert a newly discovered fragment and update reconstruction progress.
    pub fn insert(&mut self, fragment: AeonFragment) {
        self.current_known.insert(fragment.id, fragment);
        let count = self.current_known.len() as f32;
        // Rough estimation: assume 256 fragments needed for full code.
        self.reconstructed_level = (count / 256.0).min(1.0);
    }
}
