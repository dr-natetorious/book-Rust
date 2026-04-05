#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BOOK_FILE="$ROOT_DIR/chapters/book.adoc"

asciidoctor -v -a reproducible "$BOOK_FILE" -o /dev/null >/dev/null
echo "AsciiDoc includes validated"
