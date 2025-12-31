#!/usr/bin/env bash
# SCRIPT: run.sh
# DESCRIPTION: Build the release binary and run it against a local test image.
# USAGE: ./run
# PARAMETERS: None
# EXAMPLE: ./run
# ----------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "$(basename "$SCRIPT_DIR")" = "scripts" ]; then
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/script-helpers}"
else
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/scripts/script-helpers}"
fi
source "$SCRIPT_HELPERS_DIR/helpers.sh"
shlib_import help logging
parse_common_args "$@"

test_image="test.jpeg"

cargo build --release
if [ $? -ne 0 ]; then
    echo "Cargo build failed"
    exit 1
fi

# run the cargo build command
cargo run --release ./src/${test_image}
if [ $? -ne 0 ]; then
    echo "Cargo run failed"
    exit 1
fi
