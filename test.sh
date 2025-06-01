#!/bin/bash
set -e

# Install dependencies (uncomment if needed)
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    libgtk-3-dev \
    libglib2.0-dev \
    libgdk-pixbuf2.0-dev \
    cargo \
    mingw-w64

# Add Rust targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin

# Build for native release
cargo build --release

mkdir -p artifacts

# Windows
cargo build --release --target=x86_64-pc-windows-gnu
if [ -f target/x86_64-pc-windows-gnu/release/image-view.exe ]; then
  cp target/x86_64-pc-windows-gnu/release/image-view.exe artifacts/image-view-windows.exe
else
  echo "Windows binary not found!" && exit 1
fi

# Linux GNU
cargo build --release --target=x86_64-unknown-linux-gnu
if [ -f target/x86_64-unknown-linux-gnu/release/image-view ]; then
  cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-linux
  cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-deb
  cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-pacman
  cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-yum
  cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-redhat
else
  echo "Linux GNU binary not found!" && exit 1
fi

# Linux MUSL
cargo build --release --target=x86_64-unknown-linux-musl
if [ -f target/x86_64-unknown-linux-musl/release/image-view ]; then
  cp target/x86_64-unknown-linux-musl/release/image-view artifacts/image-view-musl
else
  echo "Linux MUSL binary not found!" && exit 1
fi

# macOS
if command -v x86_64-apple-darwin-gcc >/dev/null 2>&1; then
  export CARGO_BUILD_TARGET_X86_64_APPLE_DARWIN_LINKER=x86_64-apple-darwin-gcc
  cargo build --release --target=x86_64-apple-darwin
  if [ -f target/x86_64-apple-darwin/release/image-view ]; then
    cp target/x86_64-apple-darwin/release/image-view artifacts/image-view-mac
  else
    echo "macOS binary not found!" && exit 1
  fi
else
  echo "macOS cross-compiler not found, skipping macOS build."
fi

ls -al artifacts
