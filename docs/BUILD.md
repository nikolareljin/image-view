# Build

## Prerequisites

- Rust toolchain (stable).
- Cargo (comes with Rust).

## Local build

From the repo root:

```bash
cargo build --release
```

Run locally:

```bash
cargo run -- ./path/to/image.png
```

Install from source:

```bash
cargo install --path .
```

## Scripts

Run from repo root unless noted.

- `./run`: lint + build + run helper.
- `./scripts/lint.sh`: formatting + clippy checks (`-f` to auto-fix).
- `./scripts/build.sh`: build helper.
- `./scripts/test.sh`: run tests.

## Docker

Build and run the Docker flow:

```bash
./build.sh <image_name> <tag>
```
