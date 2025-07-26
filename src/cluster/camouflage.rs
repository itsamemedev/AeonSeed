use bevy::prelude::*;

/// Resource representing which seeds are hidden from regular view.
#[derive(Resource, Default)]
pub struct CamouflagedSeeds {
    pub hidden: Vec<Entity>,
}

pub struct CamouflagePlugin;

impl Plugin for CamouflagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamouflagedSeeds>();
    }
}
