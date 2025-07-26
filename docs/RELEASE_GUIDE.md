# RELEASE GUIDE

Dieses Dokument beschreibt den Ablauf eines Releases von **AeonSeed**.

## Versionierung

* SemVer: `vMAJOR.MINOR.PATCH`
* Tags werden auf den entsprechenden Commit gesetzt (`git tag -a v1.2.3`)

## Ablauf

1. `just lint` und `just test` müssen grün sein
2. `just build-server` und `just build-client` ausführen
3. Artefakte packen: `just deploy`
4. GitHub Release erstellen und Binärdateien anhängen

## Patches

Fehlerbehebungen erfolgen auf einem Branch `fix/x.y.z`. Nach erfolgreichem Testen wird ein Patch-Release mit neuer Versionsnummer erstellt.

