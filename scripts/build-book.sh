#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHAPTERS_DIR="$ROOT_DIR/chapters"
BUILD_DIR="${BUILD_DIR:-$ROOT_DIR/build}"

if ! ruby -e "require 'asciidoctor-diagram'" >/dev/null 2>&1; then
	echo "asciidoctor-diagram is required to build Mermaid diagrams. Install the gem locally or run the build inside the repo container." >&2
	exit 1
fi

if ! command -v mmdc >/dev/null 2>&1; then
	echo "Mermaid CLI (mmdc) is required to build Mermaid diagrams. Install it locally or run the build inside the repo container." >&2
	exit 1
fi

mkdir -p "$BUILD_DIR/html" "$BUILD_DIR/pdf" "$BUILD_DIR/epub"

asciidoctor -r asciidoctor-diagram -a stylesheet=theme/web.css -D "$BUILD_DIR/html" "$CHAPTERS_DIR/book.adoc"
asciidoctor-pdf -r asciidoctor-diagram -a pdf-theme="$CHAPTERS_DIR/theme/pdf-theme.yml" -D "$BUILD_DIR/pdf" "$CHAPTERS_DIR/book.adoc"
asciidoctor-epub3 -r asciidoctor-diagram -D "$BUILD_DIR/epub" "$CHAPTERS_DIR/book.adoc"

echo "Built HTML, PDF, and EPUB under $BUILD_DIR"
