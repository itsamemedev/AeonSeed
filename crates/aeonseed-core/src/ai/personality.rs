use serde::{Deserialize, Serialize};

/// Describes basic personality traits influencing NPC reactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    /// Willingness to face danger.
    pub courage: f32,
    /// Tendency to distrust others.
    pub distrust: f32,
    /// Openness and kindness towards strangers.
    pub friendliness: f32,
    /// Sense of pride driving actions.
    pub pride: f32,
}

impl Default for PersonalityProfile {
    fn default() -> Self {
        Self {
            courage: 0.5,
            distrust: 0.5,
            friendliness: 0.5,
            pride: 0.5,
        }
    }
}
