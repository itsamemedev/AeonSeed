# Installation und Erste Schritte

Diese Anleitung richtet sich an Spieler*innen und Entwickler*innen. Die Installation verläuft über ein grafisches Setup und funktioniert auf Windows, Linux und macOS.

## Voraussetzungen

* **Windows**: Lade den Installer aus den [Releases](https://github.com/itsamemedev/AeonSeed/releases) herunter. Alle Abhängigkeiten werden mitgeliefert.
* **macOS**: Öffne das `.dmg`, ziehe AeonSeed in den Programme‑Ordner und starte die App.
* **Linux**: Entpacke das `.tar.gz` und führe das beigefügte Setup-Skript aus.

Während der Installation werden **MongoDB** und **Redis** automatisch eingerichtet, falls sie nicht vorhanden sind. `Rust` und `git` benötigst du nur, wenn du selbst entwickeln willst.

## Repository klonen (für Entwickler*innen)

```bash
git clone https://github.com/itsamemedev/AeonSeed.git
cd AeonSeed
```

## Projekt bauen und starten (Entwicklungsmodus)

Der Client lässt sich direkt mit `cargo run` starten:

```bash
cargo run
```

Beim ersten Start wird automatisch ein Seed erzeugt und in `./data/` abgelegt.

Nach der regulären Installation öffnet sich ein grafisches Startmenü. Dort kannst du **ohne Webseite** einen Account anlegen oder im Gastmodus starten. Zusätzlich bietet das Menü Optionen für Grafikqualität, Sprache und Barrierefreiheit.

## Audio/Voice/TTS

TTS- und Voice-Cloning-Funktionen sind optional und über Feature-Flags eingebunden. Stelle sicher, dass dein System die benötigten Bibliotheken enthält. Weitere Hinweise findest du in den Issues.

![Startmenü](../assets/start_menu.png)

## Weitere Hilfe

- [Build-Anleitung](./BUILD.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- Fragen und Probleme bitte als Issue melden.
