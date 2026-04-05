#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHAPTERS_DIR="$ROOT_DIR/chapters"
BUILD_DIR="${BUILD_DIR:-$ROOT_DIR/build}"

resolve_mmdc() {
	local detected=""
	local resolved=""

	if detected="$(command -v mmdc 2>/dev/null)"; then
		resolved="$(readlink -f "$detected" 2>/dev/null || printf '%s' "$detected")"
		if [[ "$(basename "$detected")" == "mmdc" && "$detected" != /snap/bin/* && "$resolved" != /snap/bin/* && "$(basename "$resolved")" != "snap" ]]; then
			printf '%s\n' "$resolved"
			return 0
		fi
	fi

	for candidate in /usr/local/bin/mmdc "$HOME/.npm/bin/mmdc"; do
		if [[ -x "$candidate" ]]; then
			printf '%s\n' "$candidate"
			return 0
		fi
	done

	return 1
}

DIAGRAM_ARGS=()
if ruby -e "require 'asciidoctor-diagram'" >/dev/null 2>&1; then
	DIAGRAM_ARGS=(-r asciidoctor-diagram)
	if MMDC_PATH="$(resolve_mmdc)"; then
		DIAGRAM_ARGS+=(-a "mmdc=$MMDC_PATH")
	else
		echo "Warning: compatible mmdc not found; Mermaid blocks will not render as images." >&2
	fi
else
	echo "Warning: asciidoctor-diagram not available; Mermaid blocks will be rendered as source blocks." >&2
fi

mkdir -p "$BUILD_DIR/html" "$BUILD_DIR/pdf" "$BUILD_DIR/epub"

failures=()

if ! asciidoctor "${DIAGRAM_ARGS[@]}" -a stylesheet=theme/web.css -D "$BUILD_DIR/html" "$CHAPTERS_DIR/book.adoc"; then
	failures+=("html")
	echo "Warning: HTML build failed; continuing." >&2
fi

if ! asciidoctor-pdf "${DIAGRAM_ARGS[@]}" -a pdf-theme="$CHAPTERS_DIR/theme/pdf-theme.yml" -D "$BUILD_DIR/pdf" "$CHAPTERS_DIR/book.adoc"; then
	failures+=("pdf")
	echo "Warning: PDF build failed; continuing." >&2
fi

if ! asciidoctor-epub3 "${DIAGRAM_ARGS[@]}" -D "$BUILD_DIR/epub" "$CHAPTERS_DIR/book.adoc"; then
	failures+=("epub")
	echo "Warning: EPUB build failed; continuing." >&2
fi

if [[ ${#failures[@]} -gt 0 ]]; then
	echo "Build completed with warnings under $BUILD_DIR (failed: ${failures[*]})." >&2
else
	echo "Built HTML, PDF, and EPUB under $BUILD_DIR"
fi
