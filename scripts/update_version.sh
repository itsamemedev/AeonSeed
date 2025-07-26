#!/bin/bash
TAG=$(git describe --tags --abbrev=0)
VER=${TAG#v}
sed -i "s/^version = \".*\"/version = \"$VER\"/" crates/*/Cargo.toml
