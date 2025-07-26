# BUILD

Dieses Dokument beschreibt den Build-Prozess für Server, Client und WebAssembly-Version von **AeonSeed**. Alle Aufgaben lassen sich über das `justfile` ausführen.

## Voraussetzungen

* Rust stable via `rustup`
* optionale Tools: `wasm-pack`, `trunk`
* MongoDB und Redis lokal oder remote erreichbar

## Client

```bash
just build-client        # Release-Build
just dev                 # Entwicklungsmodus mit HotReload
```

Der Client unterstützt optionale Features wie `mobile_compat` und `voicechat`.
Aktiviere sie per `cargo run --features "mobile_compat voicechat"`.

## Server

```bash
just build-server
```

Die Server-Binärdatei liegt danach in `target/release/aeonseed-server`.
Sie kann ohne `.env` gestartet werden; alle Einstellungen befinden sich in `config/*.toml`.

## WebAssembly

```bash
just wasm
```

Dies erstellt ein fertiges Paket für Browser-Clients. Das Ergebnis wird im `pkg/`-Ordner abgelegt.

## Cross-Platform

Getestet wird unter Linux, macOS und Windows. Der Build erfolgt über `rustup`-Targets, auf Wunsch auch per Cross-Compilation.

## Deployment

```bash
just deploy
```

Dadurch werden die Release-Builds gepackt und können auf einen Server hochgeladen werden.

\n### Presets\n- `--profile server-release` führt ein optimiertes Build aus\n- Headless-Modus: `aeonseed-server --no-render`
\nWASM-Builds: `just wasm`\nMobile-Preset: `cargo build --target wasm32-unknown-unknown --features mobile_compat`
