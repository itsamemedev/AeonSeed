# Modding API

Die Modding-Schnittstelle ermöglicht es, eigene Inhalte bequem zu laden. Alle Erweiterungen befinden sich in einem `mods/`‑Ordner neben der ausführbaren Datei.

## Allgemeines Prinzip
- Definitionen werden beim Start automatisch erkannt.
- Unterstützte Formate: **RON**, **TOML** und **Lua** für Skripte.
- Registriere deine Inhalte über `modding::api::ModdingRegistry`.

## Eigene Klassen
```ron
[[classes]]
name = "Chronomancer"
start_skill = "time_bolt"
```

## Custom Quests
```toml
[quests.time_trial]
title = "The Time Trial"
script = "scripts/time_trial.lua"
```

Lua-Skripte haben Zugriff auf grundlegende Spielfunktionen über das `seed_ai`‑Binding.

## Neue Seed-Zonen
```ron
[[zones]]
identifier = "crystal_caverns"
size = "medium"
```

Lege einfach weitere Dateien in den `mods/`‑Ordner, um neue Biome oder NPCs hinzuzufügen.

## Erweiterungen
- **Lua**: Ereignisse und KI‑Reaktionen skripten
- **RON/TOML**: Datenstrukturen wie Items, Klassen, Quests

Die API ist bewusst schlank gehalten und wächst mit zukünftigen Versionen.
