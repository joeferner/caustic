#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")

cd "${SCRIPT_DIR}/../examples"

for project in *; do
    echo "checking ${project}..."
    pushd "${SCRIPT_DIR}/../examples/${project}" > /dev/null
    scad_filename=$(ls *.scad | head -n 1)
    export OPENSCADPATH="${SCRIPT_DIR}/../../frontend/public/"
    openscad -o - --export-format echo "${scad_filename}"
    popd > /dev/null
done

echo "webapp/backend complete"
