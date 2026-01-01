#!/usr/bin/env bash
# SCRIPT: build.sh
# DESCRIPTION: Build the Docker image and run the release build/test flow inside the container.
# USAGE: ./build
# PARAMETERS: None
# EXAMPLE: ./build
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

BUILD_MACOS="${BUILD_MACOS:-0}"
docker build --build-arg BUILD_MACOS="$BUILD_MACOS" -t rust-project .

# Run the Docker container with the mounted volume
docker run --rm -v "$(pwd)/target/release:/app/target/release" rust-project
if [ $? -ne 0 ]; then
    echo "Docker run failed"
    exit 1
fi

# Run the test image
test_image="test.jpeg"
docker run --rm -v "$(pwd)/target/release:/app/target/release" rust-project ./target/release/image-view ./src/${test_image}
if [ $? -ne 0 ]; then
    echo "Cargo run failed"
    exit 1
fi
echo "Build and test completed successfully"
