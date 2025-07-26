# Architekturuebersicht

AeonSeed besteht aus mehreren Crates. Die wichtigsten Systeme fuer die neuen Features sind in `aeonseed-core` untergebracht.

- **Seeds (`src/seeds/`)** verwalten serverbezogene Logik. Die geplante `fusion.rs` Datei erweitert dieses Modul um Seed-Fusion.
- **Features (`src/features/`)** wird fuer Gameplay-Erweiterungen wie Gilden genutzt.
- **KI (`src/ai/`)** enthaelt Kernsysteme wie `npc_core` und zukuenftig `reputation.rs` fuer ein Rufsystem.
- **Expansions (`src/expansions/`)** bieten optionale Plugins (z.B. `seed_fusion`).

Die Server kommunizieren ueber MongoDB und Redis, definiert in `infra/`. Bevy sorgt fuer das ECS und Rendering. Neue Komponenten sollten als Plugins implementiert werden, damit sie modular aktivierbar bleiben.

Weitere Details finden sich in `docs/FEATURES_100.md` und der `ROADMAP.md`.
