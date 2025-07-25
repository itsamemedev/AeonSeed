use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Voice cloning profile describing how an NPC should sound.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCloneProfile {
    /// Base timbre or reference voice used for cloning
    pub base_voice: String,
    /// Emotional tone that influences synthesis
    pub emotional_tone: String,
    /// Optional per-language voice overrides (TTS system specific IDs)
    pub language_overrides: HashMap<String, String>,
}

impl VoiceCloneProfile {
    pub fn new<T: Into<String>>(base: T, tone: T) -> Self {
        Self { base_voice: base.into(), emotional_tone: tone.into(), language_overrides: HashMap::new() }
    }

    /// Returns the appropriate voice id for a given language if available
    pub fn voice_for(&self, lang: &str) -> &str {
        self.language_overrides
            .get(lang)
            .map(String::as_str)
            .unwrap_or(&self.base_voice)
    }
}

/// Placeholder TTS function. In production this would stream audio data.
pub fn synthesize_to_audio(_line: &str, _profile: &VoiceCloneProfile) {
    // Implementation would call into a TTS pipeline (e.g. Whisper+Bark)
}
