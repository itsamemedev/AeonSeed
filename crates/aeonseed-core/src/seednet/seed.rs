use bevy::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dna::{generate_random_seed_dna, generate_template_seeds, SeedDNA};
use super::health::SeedHealth;

/// Core component representing a world instance.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub dna: SeedDNA,
    pub health: SeedHealth,
    pub is_active: bool,
    pub last_tick: DateTime<Utc>,
}

/// Plugin creating seed instances at startup.
pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_template_seeds);
    }
}

/// Spawn a few sample seeds to demonstrate the system.
fn spawn_template_seeds(mut commands: Commands) {
    for dna in generate_template_seeds() {
        commands.spawn(Seed {
            id: Uuid::new_v4(),
            dna,
            health: SeedHealth::default(),
            is_active: true,
            last_tick: Utc::now(),
        });
    }

    // Example of generating a completely random seed.
    commands.spawn(Seed {
        id: Uuid::new_v4(),
        dna: generate_random_seed_dna(),
        health: SeedHealth::default(),
        is_active: true,
        last_tick: Utc::now(),
    });
}
