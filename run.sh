#!/bin/bash

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
