use bevy::prelude::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Possible world environments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnvironmentType {
    Desert,
    Ice,
    Swamp,
    Volcano,
    Lush,
}

/// Stage of technological and cultural development.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CivilizationStage {
    Prehistoric,
    Medieval,
    Future,
}

/// High level faction type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FactionKind {
    Tribe,
    Nomads,
    Caste,
    Empire,
}

/// DNA describing a single faction within a Seed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactionDNA {
    pub name: String,
    pub kind: FactionKind,
}

/// Whether magic can be used in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MagicRule {
    Allowed,
    Forbidden,
}

/// PvP combat permission.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PvPRule {
    Enabled,
    Disabled,
}

/// Whether death is permanent.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PermadeathRule {
    Enabled,
    Disabled,
}

/// Use of high technology like guns or machines.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TechnologyRule {
    Allowed,
    Forbidden,
}

/// Collection of world rules.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleSet {
    pub magic: MagicRule,
    pub pvp: PvPRule,
    pub permadeath: PermadeathRule,
    pub technology: TechnologyRule,
}

/// Full DNA description of a Seed.
#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct SeedDNA {
    pub environment: EnvironmentType,
    pub ruleset: RuleSet,
    /// Speed at which world time passes (1.0 = real time).
    pub time_speed: f32,
    pub civilization_stage: CivilizationStage,
    pub factions: Vec<FactionDNA>,
}

/// Plugin providing DNA helper utilities.
pub struct DnaPlugin;

impl Plugin for DnaPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Generate a random Seed DNA using simple templates.
pub fn generate_random_seed_dna() -> SeedDNA {
    let mut rng = thread_rng();
    let envs = [
        EnvironmentType::Desert,
        EnvironmentType::Ice,
        EnvironmentType::Swamp,
        EnvironmentType::Volcano,
        EnvironmentType::Lush,
    ];
    let environment = envs[rng.gen_range(0..envs.len())].clone();

    let ruleset = RuleSet {
        magic: if rng.gen_bool(0.5) {
            MagicRule::Allowed
        } else {
            MagicRule::Forbidden
        },
        pvp: if rng.gen_bool(0.5) {
            PvPRule::Enabled
        } else {
            PvPRule::Disabled
        },
        permadeath: if rng.gen_bool(0.5) {
            PermadeathRule::Enabled
        } else {
            PermadeathRule::Disabled
        },
        technology: if rng.gen_bool(0.5) {
            TechnologyRule::Allowed
        } else {
            TechnologyRule::Forbidden
        },
    };

    let stages = [
        CivilizationStage::Prehistoric,
        CivilizationStage::Medieval,
        CivilizationStage::Future,
    ];
    let civilization_stage = stages[rng.gen_range(0..stages.len())].clone();

    let faction_count = rng.gen_range(1..=3);
    let factions = (0..faction_count)
        .map(|i| FactionDNA {
            name: format!("Faction {}", i + 1),
            kind: FactionKind::Tribe,
        })
        .collect();

    SeedDNA {
        environment,
        ruleset,
        time_speed: rng.gen_range(0.5..2.0),
        civilization_stage,
        factions,
    }
}

/// Create example DNA configurations used for demonstration purposes.
pub fn generate_example_seeds() -> Vec<SeedDNA> {
    vec![
        // "Vurtan's Drift" – Eiswelt, langsame Zeit, Technologieverbot
        SeedDNA {
            environment: EnvironmentType::Ice,
            ruleset: RuleSet {
                magic: MagicRule::Allowed,
                pvp: PvPRule::Disabled,
                permadeath: PermadeathRule::Disabled,
                technology: TechnologyRule::Forbidden,
            },
            time_speed: 0.5,
            civilization_stage: CivilizationStage::Medieval,
            factions: vec![
                FactionDNA {
                    name: "Frostborn".to_string(),
                    kind: FactionKind::Empire,
                },
                FactionDNA {
                    name: "Glacial Syndicate".to_string(),
                    kind: FactionKind::Tribe,
                },
            ],
        },
        // "Ashvault" – Vulkansystem, PvP aktiviert, Nomadenfraktion
        SeedDNA {
            environment: EnvironmentType::Volcano,
            ruleset: RuleSet {
                magic: MagicRule::Allowed,
                pvp: PvPRule::Enabled,
                permadeath: PermadeathRule::Enabled,
                technology: TechnologyRule::Allowed,
            },
            time_speed: 1.0,
            civilization_stage: CivilizationStage::Prehistoric,
            factions: vec![FactionDNA {
                name: "Ash Nomads".to_string(),
                kind: FactionKind::Nomads,
            }],
        },
        // "Erelia" – blühende Agrarwelt, High-Magic-Regelwerk, 3 konkurrierende Kasten
        SeedDNA {
            environment: EnvironmentType::Lush,
            ruleset: RuleSet {
                magic: MagicRule::Allowed,
                pvp: PvPRule::Disabled,
                permadeath: PermadeathRule::Disabled,
                technology: TechnologyRule::Allowed,
            },
            time_speed: 1.2,
            civilization_stage: CivilizationStage::Future,
            factions: vec![
                FactionDNA {
                    name: "Solar Caste".to_string(),
                    kind: FactionKind::Caste,
                },
                FactionDNA {
                    name: "Lunar Caste".to_string(),
                    kind: FactionKind::Caste,
                },
                FactionDNA {
                    name: "Verdant Caste".to_string(),
                    kind: FactionKind::Caste,
                },
            ],
        },
    ]
}
