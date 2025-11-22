#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

cd "${SCRIPT_DIR}/.."
cargo fmt
cargo clippy --workspace

echo "complete!"
