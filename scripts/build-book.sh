#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHAPTERS_DIR="$ROOT_DIR/chapters"
BUILD_DIR="$ROOT_DIR/build"

mkdir -p "$BUILD_DIR/html" "$BUILD_DIR/pdf" "$BUILD_DIR/epub"

asciidoctor -r asciidoctor-diagram -a stylesheet=theme/web.css -D "$BUILD_DIR/html" "$CHAPTERS_DIR/book.adoc"
asciidoctor-pdf -a pdf-theme="$CHAPTERS_DIR/theme/pdf-theme.yml" -D "$BUILD_DIR/pdf" "$CHAPTERS_DIR/book.adoc"
asciidoctor-epub3 -D "$BUILD_DIR/epub" "$CHAPTERS_DIR/book.adoc"

echo "Built HTML, PDF, and EPUB under $BUILD_DIR"
