#!/bin/bash

# Script that builds the Rust project and tests the release.yml commands in the Github Action
# Usage: ./build.sh
# This will start the Dockerfile and run the build process in the Dockerfile
# Mount the directory ./target/release in the host machine to the Docker container
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
