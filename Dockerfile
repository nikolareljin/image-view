# Dockerfile for image-view

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    libgtk-3-dev \
    libglib2.0-dev \
    libgdk-pixbuf2.0-dev \
    curl \
    mingw-w64

# Install rustup and Rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app
COPY . .

# Add Rust targets
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add x86_64-apple-darwin

# Build for native release
RUN cargo build --release

RUN mkdir -p artifacts

# Windows
RUN cargo build --release --target=x86_64-pc-windows-gnu && \
    if [ -f target/x86_64-pc-windows-gnu/release/image-view.exe ]; then \
      cp target/x86_64-pc-windows-gnu/release/image-view.exe artifacts/image-view-windows.exe; \
    else \
      echo "Windows binary not found!" && exit 1; \
    fi

# Linux GNU
RUN cargo build --release --target=x86_64-unknown-linux-gnu && \
    if [ -f target/x86_64-unknown-linux-gnu/release/image-view ]; then \
      cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-linux && \
      cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-deb && \
      cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-pacman && \
      cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-yum && \
      cp target/x86_64-unknown-linux-gnu/release/image-view artifacts/image-view-redhat; \
    else \
      echo "Linux GNU binary not found!" && exit 1; \
    fi

# Linux MUSL
RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    if [ -f target/x86_64-unknown-linux-musl/release/image-view ]; then \
      cp target/x86_64-unknown-linux-musl/release/image-view artifacts/image-view-musl; \
    else \
      echo "Linux MUSL binary not found!" && exit 1; \
    fi

# macOS
RUN cargo build --release --target=x86_64-apple-darwin && \
    if [ -f target/x86_64-apple-darwin/release/image-view ]; then \
      cp target/x86_64-apple-darwin/release/image-view artifacts/image-view-mac; \
    else \
      echo "macOS binary not found!" && exit 1; \
    fi

RUN ls -al artifacts

ENTRYPOINT ["/bin/bash"]