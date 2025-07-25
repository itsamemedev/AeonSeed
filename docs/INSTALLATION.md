# Installation und Erste Schritte

Diese Anleitung richtet sich an Entwickler*innen, die zum Projekt beitragen möchten.

## Voraussetzungen

- **Rust**: Installiere Rust und `cargo` über [rustup.rs](https://rustup.rs).
- **Git**: Zum Klonen des Repositorys.
- Optional für Audio/TTS:
  - `libasound2-dev` und `libudev-dev` unter Linux

## Repository klonen

```bash
git clone https://github.com/itsamemedev/AeonSeed.git
cd AeonSeed
```

## Projekt bauen und starten

Der Client lässt sich direkt mit `cargo run` starten:

```bash
cargo run
```

Beim ersten Start wird automatisch ein Seed erzeugt und in `./data/` abgelegt.

## Audio/Voice/TTS

TTS- und Voice-Cloning-Funktionen sind optional und über Feature-Flags eingebunden. Stelle sicher, dass dein System die benötigten Bibliotheken enthält. Weitere Hinweise findest du in den Issues.

## Weitere Hilfe

- [Build-Anleitung](./BUILD.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- Fragen und Probleme bitte als Issue melden.
