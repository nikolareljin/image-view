#!/usr/bin/env bash
# SCRIPT: include.sh
# DESCRIPTION: Common loader for repo scripts (helpers + standard args).
# USAGE: source ./scripts/include.sh "$@"
# ----------------------------------------------------
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$ROOT_DIR/scripts/script-helpers}"

if [ ! -f "$SCRIPT_HELPERS_DIR/helpers.sh" ]; then
  echo "script-helpers is missing. Run ./update to initialize submodules." >&2
  exit 1
fi

source "$SCRIPT_HELPERS_DIR/helpers.sh"
shlib_import help logging
parse_common_args "$@"

# Prefer the rustup-managed stable toolchain over any system-package cargo.
# Only activate when both rustup and the stable toolchain are present.
if command -v rustup &>/dev/null && rustup toolchain list 2>/dev/null | grep -q '^stable'; then
  _toolchain_bin="$(rustup run stable rustc --print sysroot)/bin"
  export PATH="$_toolchain_bin:$PATH"
  unset _toolchain_bin
fi
