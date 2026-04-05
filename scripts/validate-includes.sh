#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BOOK_FILE="$ROOT_DIR/chapters/book.adoc"

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
	fi
fi

asciidoctor -v -a reproducible "${DIAGRAM_ARGS[@]}" "$BOOK_FILE" -o /dev/null >/dev/null
echo "AsciiDoc includes validated"
