set shell := ["bash", "-cu"]

dev:
    cargo watch -p aeonseed-client -x 'run --features "debug_ui seed_ai_dev"'

build-server:
    cargo build --release -p aeonseed-server

build-client:
    cargo build --release -p aeonseed-client

wasm:
    wasm-pack build crates/aeonseed-client --target web --release

lint:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all

bench:
    cargo bench --all

docs:
    cargo doc --no-deps --all-features

deploy:
    just build-server
    just build-client
    tar czf aeonseed_build.tar.gz target/release/aeonseed* assets config
    echo "Upload Archiv ..."
