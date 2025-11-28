#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

cd "${SCRIPT_DIR}/.."
cargo fmt

cargo clippy --workspace --exclude rust-raytracer-wasm -- -Dwarnings
cargo clippy -p rust-raytracer-wasm --target wasm32-unknown-unknown -- -Dwarnings

cargo build --workspace --exclude rust-raytracer-wasm
cargo build -p rust-raytracer-wasm --target wasm32-unknown-unknown

cargo test --workspace --exclude rust-raytracer-wasm

cd "${SCRIPT_DIR}/../crates/wasm"
wasm-pack build --target web --release

echo "complete!"
