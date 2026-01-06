# CI

## Workflows

- `rust.yml`: Lints and tests on push and PRs to `main` using shared CI helpers.
- `rust-scan.yml`: Rust security and dependency scanning on push and PRs to `main`.
- `release.yml`: Builds release binaries on tag pushes (`X.Y.Z`, `vX.Y.Z`, and RC tags).
- `release-tarballs.yml`: Builds macOS/Linux tarballs on tag pushes and bumps the Homebrew tap formula.
- `release-tag-gate.yml`: Verifies release branch/tag rules on PRs.
- `auto-tag-release.yml`: Auto-tags on merge of `release/*` branches to `main`.
- `wiki.yml`: Generates CLI help and publishes it to the GitHub wiki.

## Shared CI helpers

Most workflows use the `nikolareljin/ci-helpers` repository for standardized jobs and checks. If you need to change CI logic, update the helpers or override the workflow inputs in this repo.
