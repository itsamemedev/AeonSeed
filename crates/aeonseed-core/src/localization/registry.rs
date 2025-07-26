use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::translator::{LocalizedDialogue, PlayerLanguageProfile, TranslatedLine};

/// Stores all loaded dialogues and allows runtime overrides by mods.
#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct LocalizationRegistry {
    dialogues: HashMap<String, LocalizedDialogue>,
}

impl LocalizationRegistry {
    /// Insert or override a dialogue entry.
    pub fn insert<S: Into<String>>(&mut self, id: S, dialogue: LocalizedDialogue) {
        self.dialogues.insert(id.into(), dialogue);
    }

    /// Fetch a translation for a specific dialogue id and language.
    pub fn get(&self, id: &str, lang: &str) -> Option<&TranslatedLine> {
        self.dialogues.get(id).and_then(|d| d.get(lang))
    }
}

/// Resource with the current player's language settings.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivePlayerLanguage(pub PlayerLanguageProfile);
