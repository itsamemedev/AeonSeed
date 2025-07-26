# Seed AI

Die Seed-KI ist das Herzstück von AeonSeed. Sie erschafft Spielwelten, behält Erinnerungen und passt sich an das Verhalten der Spieler an.

## Aufbau
- **cluster_ai.rs** koordiniert mehrere Seeds und verteilt Ereignisse
- **world_ai.rs** verwaltet lokale NPCs und Quests
- **memory.rs** speichert Erlebnisse langfristig

## Einstiegspunkt
Im Code dient `seed_ai.rs` als Sammelstelle und re-exportiert alle KI-Module. Neue KI-Features sollten hier angebunden werden.

## Anpassungen
Entwickler können die Seed-KI über Konfigurationsdateien in `config/ai_policy.rs` steuern. Für tiefergehende Änderungen empfiehlt sich ein eigener Branch, da die KI stark mit anderen Systemen vernetzt ist.
