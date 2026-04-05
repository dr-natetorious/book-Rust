#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BOOK_FILE="$ROOT_DIR/chapters/book.adoc"

if ! ruby -e "require 'asciidoctor-diagram'" >/dev/null 2>&1; then
	echo "asciidoctor-diagram is required to validate Mermaid diagram blocks. Install the gem locally or run validation in the repo container." >&2
	exit 1
fi

if ! command -v mmdc >/dev/null 2>&1; then
	echo "Mermaid CLI (mmdc) is required to validate Mermaid diagram blocks. Install it locally or run validation in the repo container." >&2
	exit 1
fi

asciidoctor -r asciidoctor-diagram -v -a reproducible "$BOOK_FILE" -o /dev/null >/dev/null
echo "AsciiDoc includes validated"
