# Developer Guide

Dieser Leitfaden hilft neuen Mitwirkenden beim Einstieg in den Code von AeonSeed.

## Projektstruktur
- `src/` enthält alle Module des Spiels
- `server/` und `client` teilen sich denselben Code via Features
- `seed_ai.rs` bündelt den Einstiegspunkt für KI‑Funktionen

## Module im Überblick
- **account** – Registrierung und Sessions
- **world** – Biome, Ressourcen und Anomalien
- **events** – Dungeons, Raids, PvP
- **cluster** – Kommunikation zwischen Seeds
- **modding** – API für Erweiterungen

## Lokale Entwicklungsumgebung
1. Folge der [Installationsanleitung](INSTALL.md) und installiere zusätzlich `cargo-watch` für automatisches Neustarten.
2. Starte das Spiel im Entwicklungsmodus:
   ```bash
   cargo run --features dev_seed
   ```
3. Für Serverfunktionen kannst du das Binary `aeonseed-server` separat laufen lassen.

## Beispiel-Workflow: Neues PvP-Event einbauen
1. Erstelle unter `src/events` eine neue Datei `my_pvp.rs`.
2. Registriere dein Event im Modul `events/mod.rs`.
3. Füge Tests unter `tests/` hinzu (wenn vorhanden).
4. Starte `cargo check` und behebe etwaige Fehler.
5. Öffne einen Pull Request gemäß [CONTRIBUTING](CONTRIBUTING.md).

Viel Erfolg beim Entwickeln!
