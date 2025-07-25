use bevy::prelude::*;

pub mod fragment;
pub mod interpreter;
pub mod matrix;

use fragment::AeonFragment;
use interpreter::update_matrix;
use matrix::AeonCodeMatrix;

/// Plugin bundling Aeon Code systems.
pub struct AeonCodePlugin;

impl Plugin for AeonCodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AeonCodeMatrix>();
        app.add_systems(Update, update_matrix);
    }
}

/// Generate a fragment for seeds without one.
pub fn spawn_fragments_for_seeds(
    mut commands: Commands,
    seeds: Query<(Entity, &crate::seednet::seed::Seed), Without<AeonFragment>>,
) {
    for (entity, seed) in seeds.iter() {
        if rand::random::<f32>() < 0.2 {
            let fragment = AeonFragment::random_for_seed(seed.id);
            commands.entity(entity).insert(fragment);
        }
    }
}
