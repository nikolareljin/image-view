#!/usr/bin/env bash
# SCRIPT: build.sh
# DESCRIPTION: Build the Docker image and run the release build/test flow inside the container.
# USAGE: ./build
# PARAMETERS: None
# EXAMPLE: ./build
# ----------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "$(basename "$SCRIPT_DIR")" = "scripts" ]; then
  ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
else
  ROOT_DIR="$SCRIPT_DIR"
fi
source "$ROOT_DIR/scripts/include.sh" "$@"

BUILD_MACOS="${BUILD_MACOS:-0}"
RUN_LINT="${RUN_LINT:-1}"
docker build --build-arg BUILD_MACOS="$BUILD_MACOS" --build-arg RUN_LINT="$RUN_LINT" -t rust-project .

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
