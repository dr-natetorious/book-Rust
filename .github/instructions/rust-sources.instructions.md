---
applyTo: "src/**/*.rs,test/**/*.rs"
description: "Use when editing Rust source or tests for chapter-backed examples"
---

# Rust Source Rules

## Determinism and Testability

1. **Keep chapter examples deterministic and testable** — Use fixture data, not randomization or system state.
2. **Every source file included in chapters must have AsciiDoc tag comments marking excerpt boundaries** — Ensure chapter captions reference specific, identifiable regions.
   - Format: `// tag::section_name[]` and `// end::section_name[]`
   - One tag per cohesive concept; multiple tags OK in one file
   - Tags appear in `include::../src/chNN/file.rs[tag=section_name]` references in chapter prose
3. **When source is intended for chapter inclusion, keep function/type signatures stable** — Do not refactor signatures across chapters without updating chapter text references.

## Code Style and Clarity for Print

4. **Prefer readable line breaks for narrow print layouts** — Move long bounds into `where` clauses instead of inline parameters.
   - Print width constraint: ~70 characters per line at 9.5pt JetBrains Mono on 6×9 page
   - Break function signatures across lines with 4-space indent
5. **Keep `unsafe` rationale explicit with `// SAFETY:` comments** — Place adjacent to unsafe operations, explain preconditions and invariants.
   - Include assumptions that must hold for safety

## Test and Error Handling

6. **Add or update tests when source behavior changes** — All integration tests must pass before committing chapter-backed source updates.
7. **Keep error surfaces explicit and actionable** — Use named error types instead of generic `String`; support structured test assertions.
8. **For broken teaching snippets, isolate failure cause and keep the snippet minimal** — Compiler output must be legible in print with clear error attribution.

## Dependencies

9. **Avoid unnecessary dependencies in early chapters** — Prefer `std::` and minimal external crates until justified by chapter scope.
10. **Preserve fixture-driven testing** — Use deterministic hex or text fixtures in test/fixtures/ instead of generated or randomized data.
