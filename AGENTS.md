# AGENTS

Purpose: shared operating rules for authoring a project-driven Rust book with reproducible code and tests.

## Core Rules

1. Single source of truth for code lives in src only. Book chapters include source with AsciiDoc includes and tag references.
2. No inline duplicate code blocks in chapter prose when the same snippet exists in src.
3. Every chapter file must be named number_title.adoc and included from chapters/book.adoc.
4. Every chapter has runnable examples and a confidence-level test surface in test.
5. Keep financial domain context minimal and practical; prioritize Rust learning outcomes.
6. Preserve chapter callback flow: each chapter opens by referencing the previous exercise.
7. Changes must keep HTML, PDF, and EPUB buildability intact.
8. Container workflow is first-class: Dockerfile is built once, docker-entrypoint.sh dispatches chapter workflows.

## Authoring Contract

1. Proposal artifacts under .proposal are planning truth and machine-readable discovery aids for agents.
2. Book prose under chapters should reference source tags by path and explicit tag names.
3. Source files must annotate excerpt boundaries using AsciiDoc tag comments.
4. Tests should optimize for reader confidence, not maximal coverage.

## Quality Gates

1. cargo test must pass for committed chapter code.
2. AsciiDoc include references must resolve.
3. No missing tag targets in included source snippets.
