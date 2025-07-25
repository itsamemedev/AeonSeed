use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a single line of translated text and optional metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslatedLine {
    /// Translated text content
    pub text: String,
    /// Optional ID for voice synthesis overrides
    pub voice_id: Option<String>,
    /// Cultural notes or hints for modders
    pub cultural_note: Option<String>,
}

impl TranslatedLine {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into(), voice_id: None, cultural_note: None }
    }
}

/// Holds the original dialogue and language specific variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedDialogue {
    /// Source text as authored
    pub original_text: String,
    /// Map of language code -> translated line
    pub language_variants: HashMap<String, TranslatedLine>,
}

impl LocalizedDialogue {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { original_text: text.into(), language_variants: HashMap::new() }
    }

    /// Retrieve a translated line or fall back to English.
    pub fn get(&self, lang: &str) -> Option<&TranslatedLine> {
        self.language_variants
            .get(lang)
            .or_else(|| self.language_variants.get("en"))
    }
}

/// Basic player language preferences detected at runtime.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerLanguageProfile {
    pub language_code: String,
    pub prefers_voice: bool,
    pub region_influences: Vec<String>,
}

/// Naive placeholder translator that simply echoes the input.
pub fn translate_line(text: &str, target: &str) -> TranslatedLine {
    // In a real system this would call an external AI service. The service
    // would analyse idioms, style and cultural references before producing a
    // translation. Here we simply mark the line with the requested language.

    // TODO: plug in real translation backend (e.g. OpenAI, DeepL, LibreTranslate)
    TranslatedLine::new(format!("[{}] {}", target, text))
}
