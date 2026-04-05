#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<EOF
Usage: scripts/chapter.sh <chapter-id>
Example: scripts/chapter.sh 1
EOF
}

if [[ $# -ne 1 ]]; then
  usage
  exit 1
fi

CHAPTER_ID="$1"

case "$CHAPTER_ID" in
  1)
    cargo test --test integration_ch01
    ;;
  *)
    echo "Chapter $CHAPTER_ID is not scaffolded yet."
    exit 2
    ;;
esac
