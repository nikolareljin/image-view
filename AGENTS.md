# Repository Guidelines

## Project Structure & Module Organization
- `image-view/` is the Rust crate root (run Cargo commands from here).
- `image-view/src/main.rs` contains the CLI entry point and core logic.
- `image-view/Dockerfile` and `image-view/build.sh` support Docker packaging.
- `src/main_test.rs` holds a minimal test stub at the repo root.
- `artifacts/` and `target/` are build output directories.

## Build, Test, and Development Commands
Run these from `image-view/` unless noted.
- `cargo build --release`: build optimized binary.
- `cargo run -- <args>`: run the CLI locally.
- `cargo install --path .`: install locally from source into `~/.cargo/bin`.
- `cargo test`: run Rust tests (only those inside the crate).
- `./build.sh <image_name> <tag>`: build the release binary, build a Docker image, run it, then clean up (requires Docker).

## Coding Style & Naming Conventions
- Rust 2024 edition; use standard Rust formatting (4-space indent, rustfmt-compatible style).
- Naming: `snake_case` for functions/vars, `UpperCamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep modules small and focused; prefer explicit error messages for CLI output.

## Testing Guidelines
- Place unit tests in `image-view/src` using `#[cfg(test)]` or integration tests in `image-view/tests/`.
- Current stub test lives in `src/main_test.rs`; move or duplicate tests into the crate so `cargo test` picks them up.
- Name tests for behavior, e.g., `opens_image_path` or `rejects_missing_file`.

## Commit & Pull Request Guidelines
- Git history only shows an “Initial commit”; no strict convention is established yet.
- Use concise, imperative commit subjects (e.g., "Add PNG decoding"), one topic per commit.
- PRs should include: a short summary, how to run/verify, and sample CLI output or screenshots if behavior changes.
## Release Versioning
- Keep `Cargo.toml` `version` aligned with the latest release tag `X.Y.Z`.
- Release branches follow `release/X.Y.Z`, and the merge to master should be tagged `X.Y.Z`.
- Ensure the tag `X.Y.Z` matches the `Cargo.toml` version at release time.

## Release Versioning
- Keep `Cargo.toml` `version` aligned with the latest release tag `X.Y.Z`.
- Release branches follow `release/X.Y.Z`, and the merge to main should be tagged `X.Y.Z`.
- Ensure the tag `X.Y.Z` matches the `Cargo.toml` version at release time.

## Configuration & Deployment Notes
- Docker builds use `image-view/Dockerfile`; the helper script runs and removes containers/images, so don’t point it at production tags.
- If you add config files, document defaults in `image-view/README.md`.

## Script Helpers
- `scripts/script-helpers/` is a vendored git module; do not edit files inside it.
- Scripts in `scripts/` should use `script-helpers` for `-h/--help` output (via `helpers.sh`, `shlib_import help`, and `parse_common_args`).

## CI Helpers
- GitHub Actions workflows in `.github/workflows/` use the shared `ci-helpers` library.
- References should track the `@production` branch unless explicitly requested otherwise.
