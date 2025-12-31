#!/usr/bin/env bash
# SCRIPT: build.sh
# DESCRIPTION: Build the Docker image and run the release build/test flow inside the container.
# USAGE: ./build
# PARAMETERS: None
# EXAMPLE: ./build
# ----------------------------------------------------
docker build -t rust-project .

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
