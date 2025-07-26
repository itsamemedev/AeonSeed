# Entwicklungs-Workflow

Dieser Leitfaden beschreibt den kompletten Ablauf für die Arbeit am Projekt **AeonSeed**. Er richtet sich an neue Mitwirkende ebenso wie an erfahrene Rust-Entwickler.

## Onboarding: Erste Schritte

1. Repository clonen
2. `rustup` und die benötigten Tools installieren (`cargo-edit`, `cargo-watch`, `just`)
3. `just dev` ausführen, um den HotReload-Modus zu starten
4. Beispiel `dev/mini_seed.rs` kompilieren: `cargo run --example mini_seed`
5. Dokumentation lesen (`docs/DEV_GUIDE.md`, `docs/BUILD.md`)

## Branching & Commits

- Branch-Namen: `feat/<name>`, `fix/<name>`, `refactor/<name>`, `test/<name>`
- Commits folgen dem *Conventional Commits*-Schema: `feat: ...`, `fix: ...`, `chore: ...`
- Pull Requests müssen Tests, Dokumentation und CI-Pipeline prüfen

## Lokale Entwicklung

- Formatierung via `just lint`
- Debugging mit `tokio-console`, `tracing` und `bevy-inspector-egui`
- Feature-Flags aktivieren z. B. `cargo run --features "debug_ui modding_api"`
- HotReload über `cargo watch` und Assets in `assets/`

## Toolchain

| Tool | Zweck |
|-----|-----|
|`cargo`|Standard-Buildsystem|
|`cargo-edit`|Abhängigkeiten verwalten|
|`cargo-watch`|Automatisches Neustarten bei Änderungen|
|`cargo-feature`|Feature-Flags komfortabel setzen|
|`clippy`|Linting|
|`rustfmt`|Formatierung|
|`just`|Task-Runner (siehe `justfile`)|
|`trunk`/`wasm-pack`|WebAssembly-Builds|
|`valgrind`, `perf`|Profiling|

## Seed-AI

Der Einstiegspunkt für KI-Funktionen befindet sich in `src/seed_ai.rs`. Neue Module lassen sich als Bevy-Plugins ergänzen. Für ein neues PvE-Event erstelle ein Modul unter `src/events/`, registriere es im Plugin und schreibe Tests unter `tests/`.

\n### Profiling & Coverage\n- `just bench` erstellt Benchmarks\n- `cargo tarpaulin` generiert Testabdeckung\n- `TRACY=1` aktiviert das ECS-Profiling
