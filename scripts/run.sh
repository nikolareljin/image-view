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
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/script-helpers}"
else
  SCRIPT_HELPERS_DIR="${SCRIPT_HELPERS_DIR:-$SCRIPT_DIR/scripts/script-helpers}"
fi
source "$SCRIPT_HELPERS_DIR/helpers.sh"
shlib_import help logging
parse_common_args "$@"

# Process optarg parameters passed to the script.
gallery_mode=0
additional_param=""

while getopts "h?g" opt; do
  case $opt in
    h)
      show_help_and_exit "$0" "Run the release binary against a test image." ""
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
