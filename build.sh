#!/bin/bash

# Builds the cargo project and creates a Docker image
# Usage: ./build.sh <image_name> <tag>
# Example: ./build.sh my_image latest


# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <image_name> <tag>"
    exit 1
fi
# Assign arguments to variables
IMAGE_NAME=$1
TAG=$2
# Build the cargo project
cargo build --release
# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Cargo build failed"
    exit 1
fi
# Create a Docker image
docker build -t "$IMAGE_NAME:$TAG" .
# Check if the Docker build was successful
if [ $? -ne 0 ]; then
    echo "Docker build failed"
    exit 1
fi
# Print success message
echo "Docker image $IMAGE_NAME:$TAG built successfully"
# Run the Docker container
docker run -d --name "$IMAGE_NAME" "$IMAGE_NAME:$TAG"
# Check if the Docker run was successful
if [ $? -ne 0 ]; then
    echo "Docker run failed"
    exit 1
fi
# Print success message
echo "Docker container $IMAGE_NAME is running"
# Print the container logs
docker logs "$IMAGE_NAME"
# Check if the logs command was successful
if [ $? -ne 0 ]; then
    echo "Failed to get logs from container $IMAGE_NAME"
    exit 1
fi
# Print success message
echo "Logs from container $IMAGE_NAME printed successfully"
# Stop the Docker container
docker stop "$IMAGE_NAME"
# Check if the Docker stop was successful
if [ $? -ne 0 ]; then
    echo "Docker stop failed"
    exit 1
fi
# Print success message
echo "Docker container $IMAGE_NAME stopped successfully"
# Remove the Docker container
docker rm "$IMAGE_NAME"
# Check if the Docker rm was successful
if [ $? -ne 0 ]; then
    echo "Docker rm failed"
    exit 1
fi
# Print success message
echo "Docker container $IMAGE_NAME removed successfully"
# Remove the Docker image
docker rmi "$IMAGE_NAME:$TAG"
# Check if the Docker rmi was successful
if [ $? -ne 0 ]; then
    echo "Docker rmi failed"
    exit 1
fi
# Print success message
echo "Docker image $IMAGE_NAME:$TAG removed successfully"
# Print final message
echo "Build and run process completed successfully"
# Exit the script
exit 0