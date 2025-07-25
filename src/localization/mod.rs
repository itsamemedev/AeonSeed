use bevy::prelude::*;

pub mod translator;
pub mod voice;
pub mod registry;

/// Plugin registering localization resources.
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<registry::LocalizationRegistry>()
            .init_resource::<registry::ActivePlayerLanguage>();
    }
}
