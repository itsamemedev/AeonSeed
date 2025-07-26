use bevy::prelude::*;

use crate::game::ethics::{ForceAffinity, WorldEvent, WorldForce};
use crate::zones::alignment::ZoneAlignment;

/// Event describing an action that shifts moral forces.
#[derive(Debug, Event)]
pub struct ApplyForceEvent {
    pub player: Entity,
    pub zone: Option<Entity>,
    pub force: WorldForce,
    pub magnitude: f32,
}

/// Plugin that applies player choices to world state.
pub struct InfluencePlugin;

impl Plugin for InfluencePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyForceEvent>();
        app.add_systems(Update, apply_force);
    }
}

/// Modify zone alignment and player affinity when events occur.
fn apply_force(
    mut events: EventReader<ApplyForceEvent>,
    mut zones: Query<&mut ZoneAlignment>,
    mut players: Query<&mut ForceAffinity>,
) {
    for ev in events.read() {
        if let Ok(mut affinity) = players.get_mut(ev.player) {
            match ev.force {
                WorldForce::Resonance => affinity.resonance += ev.magnitude,
                WorldForce::Entropy => affinity.entropy += ev.magnitude,
            }
        }

        if let Some(zone_entity) = ev.zone {
            if let Ok(mut zone) = zones.get_mut(zone_entity) {
                let name = zone.zone.clone();
                match ev.force {
                    WorldForce::Resonance => {
                        zone.alignment.force_balance =
                            (zone.alignment.force_balance + ev.magnitude).clamp(-1.0, 1.0);
                        zone.alignment
                            .influenced_by
                            .push(WorldEvent::ZonePurified(name));
                    }
                    WorldForce::Entropy => {
                        zone.alignment.force_balance =
                            (zone.alignment.force_balance - ev.magnitude).clamp(-1.0, 1.0);
                        zone.alignment
                            .influenced_by
                            .push(WorldEvent::ZoneCorrupted(name));
                    }
                }
            }
        }
    }
}
