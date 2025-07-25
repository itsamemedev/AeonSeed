use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DNA information describing a Seed's unique configuration.
#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct SeedDNA {
    pub world_style: String,
    pub factions: Vec<String>,
    pub rules: HashMap<String, String>,
}

/// Plugin providing access to DNA resources.
pub struct DnaPlugin;

impl Plugin for DnaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SeedDNA>();
    }
}
