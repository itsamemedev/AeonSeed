# TESTING

Dieses Dokument erläutert, wie die Tests von **AeonSeed** ausgeführt werden.

## Unit- und Integrationstests

Alle Tests werden über `cargo test` gestartet. Die Test-Suite ist in `tests/` organisiert und kann Feature-Flags nutzen.

```bash
just test
```

### Beispiele

* `cargo test --features "offline_mode"`
* `cargo test --test cluster`

## Continuous Integration

Die CI-Pipeline (z. B. GitHub Actions) führt `just lint` und `just test` aus. Fehler in Clippy oder fehlende Tests führen zu einem fehlgeschlagenen Build.

