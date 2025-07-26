# Features Overview

This document lists the implemented systems and planned improvements for AeonSeed.

## Account System
- In-game registration and login handled by `AccountPlugin`
- Account data stored in MongoDB with Argon2 password hashing
- Login rate limiting via Redis
- Sessions persisted encrypted in `~/.aeonseed/session.json`
- Optional guest mode without authentication

## User Experience
- `UiPlugin` provides launcher and in-game menus
- No command line interaction required
- Auto setup of new seeds on first start
- Multilingual user interface support
- Controller and mouse input handled by Bevy

## Events
- Modular event plugins for dungeons, raids, PvE and PvP instances
- Dynamic instancing prepared via `DungeonPlugin`, `RaidPlugin`, `PvpPlugin` and `RitualPlugin`
- Loot and boss logic will later use Seed metadata

## Infrastructure
- `InfraPlugin` establishes Redis connections
- Placeholder updater system for future patch distribution

## Planned Improvements
1. NPC memory with long term effects
2. World calendar with astrological influence
3. Mythic mounts with bonding and skills
4. Layered world map with interactive overlays
5. Targeting system based on player profile
6. Voice chat with live TTS translation
7. Seed explorer tool for cluster data
8. Platform independent JSON state format
9. Accessible UI with screen reader support
10. Crossplay preparation for web and mobile
11. Auto updater with patch notes
12. GUI installer for all platforms
13. Crafting system with combinable professions
14. PvP deception mechanics (e.g. disguises)
15. Music and weather driven by world resonance
16. Dungeon mutation when repeatedly failed
17. Ritual system opening events
18. Repeatable events with reward debuff
19. Seed rescue when worlds decay
20. Cluster heatmap showing activity zones

## New Experimental Systems
- `world::climate` – dynamic seasons and weather affecting combat and farming
- `world::resource_pressure` – scarcity influences NPC behavior and trade
- `instance::shadow_realms` – inverted-logic versions of existing seeds
- `world::anomalies` – time acceleration or reversal events
- `lang::language_barriers` – faction languages requiring research
- `class::morphing` – ritual based class switching
- `class::shadow_classes` – hidden questline professions
- `soul::modifications` – configurable soul attributes
- `soul::reincarnation` – new worldview after death
- `skills::mutation` – skills evolve through instability or choice
- `clan::fortresses` – territory control with buff zones
- `events::pvpve` – arenas combining PvP and PvE encounters
- `clan::nomads` – traveling clans without home seeds
- `cluster::magnetism` – cross-seed collisions causing chaos
- `cluster::camouflage` – seeds hidden unless revealed by rituals
- `tools::cluster_dashboard` – web UI for cluster heatmaps
- `modding::api` – load custom classes, items and quests
- `config::ai_policy` – JSON policy loader for AI behavior
- `reporting::feedback` – screenshot and log upload button
- `audio::mood_engine` – AI-synthesized music based on mood

