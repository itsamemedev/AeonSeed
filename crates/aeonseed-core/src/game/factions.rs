use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::ethics::WorldForce;
use crate::ai::memory::MemoryModel;

/// DNA blueprint describing a faction and its doctrine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionDNA {
    /// Public name of the faction.
    pub name: String,
    /// Moral alignment guiding decisions.
    pub doctrine: WorldForce,
    /// Rules controlling how memories are stored and forgotten.
    pub memory_rules: MemoryModel,
    /// Attitude toward known players.
    pub relation_to_players: HashMap<Uuid, f32>,
}

impl Default for FactionDNA {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
            doctrine: WorldForce::Resonance,
            memory_rules: MemoryModel::default(),
            relation_to_players: HashMap::new(),
        }
    }
}

/// Component linking an entity to faction data.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    pub dna: FactionDNA,
}

/// Plugin registering faction components.
pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, _app: &mut App) {}
}
