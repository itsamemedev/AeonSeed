# Build-Anleitung

Dieser Leitfaden beschreibt, wie du Client und Server von AeonSeed kompilierst.
Sowohl Client als auch Server setzen auf **Rust** und `cargo`. Optional kannst du den Client auch als WebAssembly (wasm) bauen.

## 🧱 Client-Build

- Debug-Start:
  ```bash
  cargo run
  ```
- Release-Build:
  ```bash
  cargo build --release
  ```
- Optional: TTS/VoiceCloning über Feature-Flags aktivieren:
  ```bash
  cargo run --features voice
  ```

Für den Web-Build (WASM) installiere zunächst das Ziel:
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

## 🧱 Server-Build

Der Server befindet sich im separaten Binary `aeonseed-server` unter `src/server`.

- Starten:
  ```bash
  cargo run --bin aeonseed-server
  ```
  Konfiguration erfolgt über eine `.env`-Datei oder `config.yaml` im Projektverzeichnis.
  Der Server unterstützt Cluster-Kommunikation, Datenpersistenz und Seed-Hosting.
  MongoDB und Redis müssen erreichbar sein. Nutze `docker-compose up` oder die automatischen Installer, um beides lokal zu starten.

Für Selbsthosting kannst du in `config/cluster.toml` Seed-Knoten und Cluster-Größe definieren.

## 📦 Build-Ziele

- Erstelle nach dem Build ein ZIP-Archiv mit den Ordnern `bin/`, `assets/` und `config/`.
- Client und Server lassen sich parallel betreiben, auch innerhalb von Docker-Containern.
- Getestet auf Linux, macOS und Windows.

### Lokale Tests

```bash
cargo run --features dev_seed
```
Damit wird ein lokaler Seed inklusive Testdaten erzeugt.

### systemd-Beispiel

```ini
[Unit]
Description=AeonSeed Server
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/aeonseed-server
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

