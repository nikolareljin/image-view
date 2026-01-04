# Changelog

All notable changes to this project are documented in this file.
This format is based on Keep a Changelog and follows Semantic Versioning.

## [0.5.0] - 2025-09-18
### Changed
- Fix gallery rendering alignment by using explicit CRLF line endings in raw mode.
- Show full path beneath gallery images with dynamic height adjustment.
- Improve clipboard handling across environments (X11 owner retention, WSL/macOS/Wayland/OSC 52 fallbacks).
- Fix ASCII render width by doubling monospace characters to match block aspect ratio.

## [0.2.0] - 2025-08-30
### Changed
- Update Wiki page with the latest parameters of the main program source code (./image-view executable when being compiled and built). Push the updated information into `CLI.md` file.
- Update color printing with `colored` Rust package. This is to eliminate bash-style printing which does not work in Windows.
- Switch CI/release workflows to `ci-helpers` shared workflows for build, tagging, and release prep.
- Add `release-tag-gate` and `rust-scan` workflows from `ci-helpers`.

## [0.1.0-RC] - 2025-06-01
### Added
- Release candidate build tagged for 0.1.0.

## [0.1.0] - 2025-05-26
### Added
- Rust script to display local image with full path: `image-view <file path>`
- Update dependency `actions/checkout@v4`
- Update dependency `actions-rs/toolchain@v1`
- Auto-generate executable files for different platforms when the code gets tagged.
- Add Dockerfile, build.sh and test.sh scripts to verify building of the artifacts
- Add Release info after tagging and release GH Action completes building artifacts
- Add links to built artifacts in the Release information after the process completes.

[Unreleased]: https://github.com/nikolareljin/image-view/compare/0.5.0...HEAD
[0.5.0]: https://github.com/nikolareljin/image-view/releases/tag/0.5.0
[0.2.0]: https://github.com/nikolareljin/image-view/releases/tag/0.2.0
[0.1.0-RC]: https://github.com/nikolareljin/image-view/releases/tag/0.1.0-RC
[0.1.0]: https://github.com/nikolareljin/image-view/releases/tag/0.1.0
