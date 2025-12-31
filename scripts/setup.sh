#!/usr/bin/env bash
# SCRIPT: setup
# DESCRIPTION: Build a release binary and install it into ~/.local/bin as image-view.
# USAGE: ./setup
# NOTE: If invoked via a symlink from repo root, the script uses the symlink path
#       as the root; otherwise it resolves the repo root from scripts/.
# PARAMETERS:
#   INSTALL_DIR   : Optional override for install dir (default: ~/.local/bin).
#   PROFILE_FILE  : Optional override for profile file (default: ~/.profile).
# EXAMPLE: ./setup
# ----------------------------------------------------
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# If run via a symlink from repo root, use that path; otherwise jump up from scripts/.
if [ "$(basename "$SCRIPT_DIR")" = "scripts" ]; then
  ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
else
  ROOT_DIR="$SCRIPT_DIR"
fi
SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$ROOT_DIR/scripts/script-helpers}"
source "$SCRIPT_HELPERS_DIR/helpers.sh"
shlib_import help logging
parse_common_args "$@"
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
