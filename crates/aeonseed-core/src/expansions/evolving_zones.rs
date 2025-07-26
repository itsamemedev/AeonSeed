use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ZoneEvolution {
    pub stage: u32,
}

pub struct EvolvingZonesPlugin;

impl Plugin for EvolvingZonesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZoneEvolution>();
    }
}
