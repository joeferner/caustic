#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

cd "${SCRIPT_DIR}/.."

cargo build --workspace --exclude rust-raytracer-wasm --release
flamegraph -o flamegraph.svg -- target/release/rust-raytracer-cli

echo "complete!"
