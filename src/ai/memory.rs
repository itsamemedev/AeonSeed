use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::personality::PersonalityProfile;

/// Entity, location or event a memory is about.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemorySubject {
    /// Memory about a specific player identified by UUID.
    Player(Uuid),
    /// Memory about a location in the world.
    Location(String),
    /// Any other event description.
    Event(String),
}

/// Single memory entry with timestamp and perceived impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub timestamp: DateTime<Utc>,
    pub subject: MemorySubject,
    /// Free form details remembered by the NPC.
    pub detail: String,
    /// How strongly this memory influenced the NPC.
    pub impact: f32,
}

/// Collection of memories for a single NPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCMemory {
    /// Events from roughly the last 24 hours of play.
    pub short_term: Vec<MemoryEntry>,
    /// Persistent memories kept for the life of the NPC.
    pub long_term: Vec<MemoryEntry>,
    /// Feelings toward players, locations or factions.
    pub emotions: HashMap<String, f32>,
    /// Basic personality traits influencing reactions.
    pub personality: PersonalityProfile,
}

impl Default for NPCMemory {
    fn default() -> Self {
        Self {
            short_term: Vec::new(),
            long_term: Vec::new(),
            emotions: HashMap::new(),
            personality: PersonalityProfile::default(),
        }
    }
}

impl NPCMemory {
    /// Remember a new event either in short or long term memory.
    pub fn remember(&mut self, entry: MemoryEntry, long_term: bool) {
        const MAX_SHORT: usize = 1000;
        const MAX_LONG: usize = 1000;

        if long_term {
            self.long_term.push(entry);
            if self.long_term.len() > MAX_LONG {
                self.long_term.remove(0);
            }
        } else {
            self.short_term.push(entry);
            if self.short_term.len() > MAX_SHORT {
                self.short_term.remove(0);
            }
        }
    }
}

/// Global storage of memory per NPC entity.
#[derive(Resource, Default)]
pub struct NpcMemoryBank {
    pub memories: HashMap<Entity, NPCMemory>,
}

/// Plugin handling NPC memory graph.
pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NpcMemoryBank>();
    }
}
