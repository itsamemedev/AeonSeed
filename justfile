set shell := ["bash", "-cu"]

dev:
    cargo watch -x 'run --features "debug_ui seed_ai_dev"'

build-server:
    cargo build --release --bin aeonseed-server

build-client:
    cargo build --release

wasm:
    wasm-pack build --target web --release

lint:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all

docs:
    cargo doc --no-deps --all-features

deploy:
    just build-server
    just build-client
    tar czf aeonseed_build.tar.gz target/release/aeonseed* assets config
    echo "Upload Archiv ..." # Hier Upload-Script einbinden
