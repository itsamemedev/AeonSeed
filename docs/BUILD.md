# Build-Anleitung

Dieser Leitfaden beschreibt, wie du Client und Server von AeonSeed kompilierst.

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

## 🧱 Server-Build

Der Server befindet sich im separaten Binary `aeonseed-server` unter `src/server`.

- Starten:
  ```bash
  cargo run --bin aeonseed-server
  ```
- Konfiguration erfolgt über eine `.env`-Datei oder `config.yaml` im Projektverzeichnis.
- Der Server unterstützt Cluster-Kommunikation, Datenpersistenz und Seed-Hosting.

## 📦 Build-Ziele

- Erstelle nach dem Build ein ZIP-Archiv mit den Ordnern `bin/`, `assets/` und `config/`.
- Client und Server lassen sich parallel betreiben, auch innerhalb von Docker-Containern.
- Getestet auf Linux, macOS und Windows.

