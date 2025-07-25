use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player behavior metrics determining emergent archetypes.
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct SoulProfile {
    /// Weight for each trait accumulated through actions.
    pub tendencies: HashMap<TraitType, f32>,
    /// Tags collected from notable events.
    pub tags: Vec<String>,
    /// Current archetype derived from dominant traits.
    pub alignment: Option<EmergentArchetype>,
}

impl Default for SoulProfile {
    fn default() -> Self {
        Self {
            tendencies: HashMap::new(),
            tags: Vec::new(),
            alignment: None,
        }
    }
}

/// Core behavioral categories used for progression analysis.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TraitType {
    Combat,
    Diplomacy,
    Healing,
    Taming,
    Discovery,
    Corruption,
    Resistance,
}

/// Archetypes that can manifest from behavior patterns.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmergentArchetype {
    /// Wanderer between worlds focused on exploration.
    Fluxwarden,
    /// Warrior born from destruction and trauma.
    Ashborn,
    /// Master of lost technologies.
    Cipherwalker,
    /// Guardian of living ecosystems.
    Bloomcaller,
}

/// Event emitted when a profile analysis suggests a new archetype.
#[derive(Debug, Event)]
pub struct SoulAnalysisResult {
    pub entity: Entity,
    pub new_archetype: Option<EmergentArchetype>,
}

/// Plugin for soul profile management.
pub struct SoulPlugin;

impl Plugin for SoulPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoulAnalysisResult>();
        app.add_systems(Update, apply_soul_analysis);
    }
}

/// Update profiles when analysis results are produced by the AI.
fn apply_soul_analysis(
    mut events: EventReader<SoulAnalysisResult>,
    mut query: Query<&mut SoulProfile>,
) {
    for event in events.read() {
        if let Ok(mut profile) = query.get_mut(event.entity) {
            profile.alignment = event.new_archetype.clone();
        }
    }
}

