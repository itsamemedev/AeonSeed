use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Fundamental moral forces influencing everything in AeonSeed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldForce {
    /// Harmonie, Aufbau und Erinnerung.
    Resonance,
    /// Chaos, Zersetzung und Vergessen.
    Entropy,
}

/// Event types that can shift the moral balance of a zone or world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    PlayerCleansesRelic,
    PlayerCorruptsRelic,
    ZonePurified(String),
    ZoneCorrupted(String),
}

/// Personal tendency of a player toward each force.
#[derive(Debug, Clone, Serialize, Deserialize, Component, Default)]
pub struct ForceAffinity {
    /// Accumulated resonance through harmonious actions.
    pub resonance: f32,
    /// Accumulated entropy through destructive actions.
    pub entropy: f32,
}

/// Alignment data tracked for world regions.
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct WorldAlignment {
    /// Current balance from -1.0 (pure Entropy) to 1.0 (pure Resonance).
    pub force_balance: f32,
    /// Log of events that influenced this state.
    pub influenced_by: Vec<WorldEvent>,
}

impl Default for WorldAlignment {
    fn default() -> Self {
        Self {
            force_balance: 0.0,
            influenced_by: Vec::new(),
        }
    }
}

/// Plugin maintaining world ethics resources.
pub struct EthicsPlugin;

impl Plugin for EthicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldAlignment>();
    }
}
