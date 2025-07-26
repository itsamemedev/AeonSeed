use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BattleStats {
    pub wins: u32,
    pub losses: u32,
}

pub struct BalancingPlugin;

impl Plugin for BalancingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleStats>();
    }
}
