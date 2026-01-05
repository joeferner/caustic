#!/bin/bash
set -e

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "${SCRIPT_DIR}/.."

if [ -f /app/openapi.json ]; then
    echo "using /app/openapi.json"
    cp /app/openapi.json /tmp/openapi.json
else
    echo "generating new openapi.json"
    "${SCRIPT_DIR}/../../../target/debug/caustic-webapp" --write-swagger /tmp/openapi.json
fi

rm -rf ./src/api
npx openapi-generator-cli generate \
  -i /tmp/openapi.json \
  -o ./src/api \
  -g typescript-fetch

SED_COMMAND='1s;^;// @ts-nocheck\n;'
if [ "$(uname)" != "Darwin" ]; then
  sed -i "$SED_COMMAND" ./src/api/**/*.ts
else
  sed -i '' "$SED_COMMAND" ./src/api/**/*.ts
fi

echo "complete!"
