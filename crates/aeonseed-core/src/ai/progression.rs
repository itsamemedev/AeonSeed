use std::time::Duration;

use bevy::prelude::*;

use crate::game::soul::{EmergentArchetype, SoulAnalysisResult, SoulProfile, TraitType};

/// Timer resource controlling how often soul profiles are analyzed.
#[derive(Resource)]
pub struct ProgressionTimer(pub Timer);

impl Default for ProgressionTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(600), TimerMode::Repeating))
    }
}

/// Plugin triggering soul progression analyses.
pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProgressionTimer>();
        app.add_systems(Update, analyze_souls);
    }
}

/// Evaluate traits and emit events to update archetypes.
fn analyze_souls(
    time: Res<Time>,
    mut timer: ResMut<ProgressionTimer>,
    query: Query<(Entity, &SoulProfile)>,
    mut results: EventWriter<SoulAnalysisResult>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    for (entity, profile) in query.iter() {
        let mut dominant: Option<(TraitType, f32)> = None;
        for (t, v) in profile.tendencies.iter() {
            if let Some((_, max)) = dominant {
                if *v > max {
                    dominant = Some((*t, *v));
                }
            } else {
                dominant = Some((*t, *v));
            }
        }

        let new_archetype = match dominant.map(|p| p.0) {
            Some(TraitType::Discovery) if profile.tags.contains(&"artifact".to_string()) => {
                Some(EmergentArchetype::Cipherwalker)
            }
            Some(TraitType::Discovery) => Some(EmergentArchetype::Fluxwarden),
            Some(TraitType::Combat) if profile.tags.contains(&"trauma".to_string()) => {
                Some(EmergentArchetype::Ashborn)
            }
            Some(TraitType::Healing) => Some(EmergentArchetype::Bloomcaller),
            _ => None,
        };

        results.send(SoulAnalysisResult { entity, new_archetype });
    }
}

