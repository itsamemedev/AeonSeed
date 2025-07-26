pub mod language_barriers;

pub struct LangPlugin;

impl bevy::prelude::Plugin for LangPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((language_barriers::LanguageBarrierPlugin,));
    }
}
