use bevy::prelude::*;

pub mod auto_translate;
pub mod voice;

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, _app: &mut App) {
        // Localization setup goes here
    }
}
