pub mod mutation;

pub struct SkillsPlugin;

impl bevy::prelude::Plugin for SkillsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((mutation::SkillMutationPlugin,));
    }
}
