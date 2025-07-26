# Seed Systeme

Dieses Dokument gibt einen Überblick über die wichtigsten Seed-bezogenen Module in AeonSeed. Jedes Modul wird als Bevy-Plugin implementiert und kann eigenständig aktiviert werden.

- `seednet` – Verwaltung und Persistenz einzelner Seeds
- `cluster` – globale Koordination mehrerer Seeds
- `world` – Wetter, Ressourcen und Anomalien
- `expansions` – optionale Features wie Dynamische Events oder Seed-Fusion

Alle Plugins können beliebig kombiniert werden, um eigene Server-Setups zu erzeugen.
