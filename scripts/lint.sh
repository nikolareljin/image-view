#!/usr/bin/env bash
# SCRIPT: lint.sh
# DESCRIPTION: Run Rust formatting and clippy lint checks.
# USAGE: ./lint
# PARAMETERS:
#  -h           : Show help message and exit.
#  -f           : Fix formatting with rustfmt before linting.
# EXAMPLE: ./lint
# ----------------------------------------------------
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "$(basename "$SCRIPT_DIR")" = "scripts" ]; then
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/script-helpers}"
else
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/scripts/script-helpers}"
fi
source "$SCRIPT_HELPERS_DIR/helpers.sh"
shlib_import help logging
parse_common_args "$@"

fix_mode=0
while getopts "hf?" opt; do
  case $opt in
    h)
      show_help_and_exit "$0" "Run rustfmt and clippy checks." ""
      ;;
    f)
      fix_mode=1
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ "$fix_mode" -eq 1 ]; then
  echo "Running rustfmt"
  (
    cd "$ROOT_DIR"
    cargo fmt --all
  )
else
  echo "Running rustfmt check"
  (
    cd "$ROOT_DIR"
    cargo fmt --all -- --check
  )
fi

echo "Running clippy"
(
  cd "$ROOT_DIR"
  cargo clippy --all-targets --all-features -- -D warnings
)
