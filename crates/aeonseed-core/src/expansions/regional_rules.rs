use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RegionalRules {
    pub pvp_allowed: bool,
}

pub struct RegionalRulesPlugin;

impl Plugin for RegionalRulesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RegionalRules>();
    }
}
