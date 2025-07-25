use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Medium through which a fragment manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FragmentMedium {
    /// Visual glyphs or symbols embedded in objects.
    Glyph,
    /// Melodic or rhythmic patterns carrying meaning.
    Songline,
    /// Architectural formations hinting at code structure.
    StructurePattern,
    /// Fragments perceived in shared dreams.
    DreamSequence,
    /// Vision relayed through an NPC encounter.
    NPCVision,
}

/// Rule describing how a fragment can be interpreted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretationRule {
    /// Short hint or symbolic association.
    pub hint: String,
}

/// Single piece of the global Aeon Code.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AeonFragment {
    /// Unique identifier of the fragment.
    pub id: Uuid,
    /// Encrypted or obfuscated fragment data.
    pub encoded_data: String,
    /// Seed in which the fragment originated.
    pub origin_seed: Uuid,
    /// Representation medium of the fragment.
    pub medium: FragmentMedium,
    /// Rules guiding symbolic interpretation of the fragment.
    pub interpretation_rules: Vec<InterpretationRule>,
}

impl AeonFragment {
    /// Convenience constructor generating a random fragment bound to a seed.
    pub fn random_for_seed(origin_seed: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            encoded_data: format!("{:x}", rand::random::<u128>()),
            origin_seed,
            medium: FragmentMedium::Glyph,
            interpretation_rules: vec![InterpretationRule {
                hint: "resonance".to_string(),
            }],
        }
    }
}
