# Copilot Instructions

This repository authors a Rust book with chapter prose in AsciiDoc and source-of-truth code in src.

## Phase 1 Style Compliance (Baseline for All Chapters)

1. **Chapter headings use the format `== Chapter N: Title`** — Consistent across all chapters.
2. **Every concept-bearing code listing has a caption** — `.Caption text` line directly above `[source,*]` blocks.
3. **Each chapter has 1-2 strategic WARNING or NOTE admonitions** — Highlighting friction points specific to the chapter topic.
4. **Source block language tags are explicit** — `[source,rust]`, `[source,text]`, `[source,shell]`, `[source,toml]`.
5. **PDF theme and web CSS match O'Reilly production standards** — See chapters/theme/.
6. **All tests pass** — cargo test --all must complete without errors.

## Core Rules (Enforced on Every Change)

7. Never duplicate source snippets inline when a tagged include is available.
8. Keep chapter narrative practical and callback-driven.
9. Add or update tests when source behavior changes.
10. Preserve container reproducibility and simple reader workflows.
11. Use explicit cross-references (`<<section-id>>`) instead of directional phrases.
12. Break long Rust signatures across lines with `where` clauses for print layout.

## Content Preservation Guardrails

13. Never replace chapter prose with condensed summaries, bullet recaps, or "cleaned" short-form rewrites unless the user explicitly requests abridgement.
14. If the user asks for build/debug/tooling help, do not rewrite chapter narrative as part of that task.
15. Before any large prose rewrite, compare word count and section count against the current file and keep parity unless the user explicitly asks to shorten.
16. If a chapter is accidentally condensed, restore from the last known-good commit and report the restore before continuing.
17. For "I just need output now" requests, produce the requested artifact first; defer instruction/pipeline refactors to a separate, explicit follow-up.
