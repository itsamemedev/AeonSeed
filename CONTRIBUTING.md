# Beitrag leisten

Vielen Dank für dein Interesse an AeonSeed!

## Entwickler-Onboarding

1. Folge der [Installationsanleitung](./docs/INSTALL.md).
2. Erstelle einen Fork und einen Branch pro Feature oder Bugfix.
3. Führe vor jedem Commit `cargo check` aus.
4. Beachte unseren Code-Stil: Snake_case für Dateinamen und Funktionen, CamelCase für Typen.

## Pull Requests

- Kleine, klar abgegrenzte Änderungen bevorzugt.
- Beschreibe in der PR-Beschreibung kurz, was geändert wurde.
- Verweise auf zugehörige Issues.
- Achte darauf, dass dein Branch aktuell mit `main` rebaset wurde.

## Issues

- Nutze aussagekräftige Titel und beschreibe dein Problem möglichst genau.
- Feature-Wünsche sind willkommen, sollten aber gut begründet werden.
- Nutze die bereitgestellten Issue-Vorlagen (Bug, Feature, Frage).

## Branch-Strategie

- `main` enthält stets den aktuellen Entwicklungsstand.
- Feature-Branches werden per PR in `main` gemergt.

### Beispiel: Neue Klasse hinzufügen
1. Lege in `mods/` eine Datei `my_class.ron` an.
2. Registriere die Datei im `ModdingRegistry`.
3. Schreibe einen kurzen Test unter `tests/`.

Danke für deinen Beitrag!
