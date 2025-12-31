#!/usr/bin/env bash
# SCRIPT: setup
# DESCRIPTION: Build a release binary and install it into ~/.local/bin as image-view.
# USAGE: ./setup
# PARAMETERS:
#   INSTALL_DIR   : Optional override for install dir (default: ~/.local/bin).
#   PROFILE_FILE  : Optional override for profile file (default: ~/.profile).
# EXAMPLE: ./setup
# ----------------------------------------------------
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
PROFILE_FILE="${PROFILE_FILE:-$HOME/.profile}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Cargo is required. Install it via rustup: https://rustup.rs" >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR"

cargo build --release --manifest-path "$ROOT_DIR/Cargo.toml"

BIN_PATH="$ROOT_DIR/target/release/image-view"
if [ ! -f "$BIN_PATH" ]; then
  echo "Built binary not found at $BIN_PATH" >&2
  exit 1
fi

install -m 0755 "$BIN_PATH" "$INSTALL_DIR/image-view"
echo "Installed image-view to $INSTALL_DIR/image-view"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE_FILE"
    echo "Added $INSTALL_DIR to PATH in $PROFILE_FILE"
    echo "Restart your shell or run: source $PROFILE_FILE"
    ;;
esac
