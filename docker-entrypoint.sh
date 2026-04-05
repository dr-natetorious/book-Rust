#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<EOF
Commands:
  help                 Show this help
  test-all             Run all Rust tests
  chapter <id>         Run chapter-specific tests
  build-book           Build HTML/PDF/EPUB book outputs
EOF
}

if [[ $# -eq 0 ]]; then
  show_help
  exit 0
fi

case "$1" in
  help)
    show_help
    ;;
  test-all)
    cargo test
    ;;
  chapter)
    if [[ $# -ne 2 ]]; then
      echo "chapter requires an id"
      exit 1
    fi
    scripts/chapter.sh "$2"
    ;;
  build-book)
    scripts/build-book.sh
    ;;
  *)
    echo "Unknown command: $1"
    show_help
    exit 1
    ;;
esac
