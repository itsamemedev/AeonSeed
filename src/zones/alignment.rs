use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::ethics::WorldAlignment;

/// Alignment signature for a specific zone.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ZoneAlignment {
    /// Display name of the zone.
    pub zone: String,
    /// Moral balance affecting everything within.
    pub alignment: WorldAlignment,
}

impl Default for ZoneAlignment {
    fn default() -> Self {
        Self {
            zone: "Unnamed".to_string(),
            alignment: WorldAlignment::default(),
        }
    }
}

/// Helper describing notable default zones.
pub fn example_zones() -> Vec<ZoneAlignment> {
    vec![
        ZoneAlignment {
            zone: "Tempel der Fl\u{00fc}sternden Erinnerung".to_string(),
            alignment: WorldAlignment {
                force_balance: 0.9,
                influenced_by: Vec::new(),
            },
        },
        ZoneAlignment {
            zone: "Die Glasw\u{00fc}ste".to_string(),
            alignment: WorldAlignment {
                force_balance: -1.0,
                influenced_by: Vec::new(),
            },
        },
    ]
}

/// Plugin setting up zone alignment resources.
pub struct ZoneAlignmentPlugin;

impl Plugin for ZoneAlignmentPlugin {
    fn build(&self, _app: &mut App) {}
}
