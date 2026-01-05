#!/usr/bin/env bash
# SCRIPT: run.sh
# DESCRIPTION: Build the release binary and run it against a local test image.
# USAGE: ./run
# PARAMETERS:
#  -h           : Show help message and exit.
#  [-g [<dir>]] : Gallery | Gallery with directory
# EXAMPLE: ./run
# ----------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "$(basename "$SCRIPT_DIR")" = "scripts" ]; then
  ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
else
  ROOT_DIR="$SCRIPT_DIR"
fi
source "$ROOT_DIR/scripts/include.sh" "$@"

# Process optarg parameters passed to the script.
gallery_mode=0
additional_param=""

while getopts "h?g" opt; do
  case $opt in
    h)
      show_help_and_exit "$0" "Run the release binary against a test image." \
        "PARAMETERS:\n  -h           : Show help message and exit.\n  -g [<dir>]   : Gallery | Gallery with directory"
      additional_param="-h"
      ;;
    g)
      gallery_mode=1
      additional_param="-g"
      # Check if there's a non-option argument that could be the directory
      shift $((OPTIND - 1))
      if [ $# -gt 0 ] && [[ "$1" != -* ]]; then
        gallery_dir="$1"
        additional_param="$additional_param \"${gallery_dir}\""
        shift
      else
        gallery_dir=""
      fi
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2 
      exit 1
      ;;
  esac
done

test_image="test.jpeg"

(
  cd "$ROOT_DIR"
  ./scripts/lint.sh
)
if [ $? -ne 0 ]; then
    echo "Linting failed"
    exit 1
fi

# Reformat with cargo fmt --
cargo fmt --
if [ $? -ne 0 ]; then
    echo "Linting failed"
    exit 1
fi

# Cargo build
cargo build --release
if [ $? -ne 0 ]; then
    echo "Cargo build failed"
    exit 1
fi

# run the cargo build command
# If in gallery mode, pass the gallery flag and optional directory
if [[ $gallery_mode == 1 ]]; then
  echo "Running in gallery mode"
  if [ -n "$gallery_dir" ]; then
    cargo run --release -- -g "$gallery_dir"
  else
    cargo run --release -- -g
  fi
else
  echo "Running in single image mode"
  cargo run --release -- "./src/${test_image}"
fi

if [ $? -ne 0 ]; then
    echo "Cargo run failed"
    exit 1
fi
