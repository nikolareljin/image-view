#!/usr/bin/env bash
# SCRIPT: update
# DESCRIPTION: Sync and update git submodules, even if the repo was cloned without them.
# USAGE: ./update
# PARAMETERS: None
# EXAMPLE: ./update
# ----------------------------------------------------
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if ! command -v git >/dev/null 2>&1; then
  echo "Git is required to update submodules." >&2
  exit 1
fi

cd "$ROOT_DIR"

# Ensure submodules are initialized and updated, even if the repo was cloned without them.
git submodule sync --recursive
git submodule update --init --recursive

echo "Submodules are up to date."
