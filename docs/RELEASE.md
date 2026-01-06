# Release

## Versioning

- Keep `Cargo.toml` `version` aligned with the release tag `X.Y.Z`.
- Release branches follow `release/X.Y.Z` or `release/vX.Y.Z`.

## Tagging

Supported tag formats:

- `X.Y.Z`
- `vX.Y.Z`
- `X.Y.Z-rcN` or `vX.Y.Z-rcN`

Manual tag:

```bash
git tag -a 0.5.0 -m "Release 0.5.0"
git push origin 0.5.0
```

Auto-tag flow:

- Merge a PR from `release/X.Y.Z` (or `release/vX.Y.Z`) into `main`.
- The auto-tag workflow creates tag `X.Y.Z`.

## Release artifacts

Two workflows publish release assets on tag pushes:

- `release.yml` uses the shared rust-release workflow to build platform binaries.
- `release-tarballs.yml` builds tar.gz assets for macOS arm64, macOS x86_64, and Linux x86_64.

## Homebrew tap

The tap repo is `nikolareljin/homebrew-tap`. The `release-tarballs.yml` workflow updates the formula automatically after a release:

- Downloads the tarball sha256 values.
- Updates `Formula/image-view.rb` with the new version and shas.
- Commits and pushes the change.

Required repository secrets in `image-view`:

- `HOMEBREW_TAP_GITHUB_TOKEN`: classic PAT with `repo` scope and write access to the tap repo.
- `HOMEBREW_TAP_REPO`: `nikolareljin/homebrew-tap`.
