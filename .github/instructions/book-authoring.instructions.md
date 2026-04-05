---
applyTo: "chapters/**/*.adoc"
description: "Use when editing chapter prose in AsciiDoc, including callback openings and tagged source includes"
---

# Book Authoring Rules

## Chapter Structure

1. **Chapter heading format must be `== Chapter N: Title`** — Apply uniformly across all chapters. The chapter number is bound to the title, not separated.
2. **Preamble is 3-5 sentences between heading and first section** — Opens with callback to previous chapter's exercise, states the problem, transitions to first section.
3. **Keep domain context to one concise paragraph** — Placed exactly where first needed, not front-loaded.
4. Start each chapter with a callback to the previous chapter exercise.

## Code Listings and Language Tags

5. **Every concept-bearing code listing MUST have a caption** — `.Caption text` line directly above the `[source,*]` block. The caption explains the listing's purpose, not its syntax.
   - Example: `.A zero-copy AddOrder: using borrowed slices instead of owned arrays`
6. **Use explicit source language tags:**
   - `[source,rust]` for Rust code
   - `[source,text]` for compiler diagnostics, errors, warnings, and program output
   - `[source,shell]` for cargo and shell commands
   - `[source,toml]` for Cargo.toml manifests
7. **For intentionally broken code** — Place a `WARNING` block above the listing explaining the bug.
8. **Use callout markers for conceptual code explanation** — Avoid inline explanatory comments inside listings. Callouts (`<1>`, `<2>`) are explained below the listing.
9. **Use include:: references to src tags for code** — Never duplicate source snippets inline. Reference tagged excerpts from src/chNN/.
10. **Use explicit cross-references (`<<anchor-id>>`)** instead of directional phrases like "as mentioned above" or "earlier in this chapter".

## Admonition Strategy

11. **Add 1-2 strategic admonitions per chapter** — Type chosen by intent:
    - `WARNING:` Safety violations, common pitfalls, or operations with irreversible consequences (most common)
    - `NOTE:` Information that breaks flow but is essential for understanding
    - `TIP:` Optional optimizations, shortcuts, or working patterns (use sparingly)
    - `IMPORTANT:` Critical preconditions or invariants
    - `CAUTION:` Potential data loss or irreversible actions
    - Each admonition addresses a friction point specific to the chapter topic.

## Storytelling and Prose

12. Favor concrete engineering tradeoffs over broad theory.
13. Teach the reader how to do the work, not about the concept in the abstract.
14. Put the reader in motion early: action first, breakage second, explanation third.
15. Treat the reader as the protagonist of the chapter. The prose tracks their decisions, errors, and progress.
16. Vary cadence on purpose: do not let sections collapse into repetitive sentence length, paragraph length, or repeated phrasing.
17. Spend page budget according to difficulty and payoff. Hard ideas get room; obvious steps move quickly.
18. Assume engineering maturity, not prior Rust fluency. Define Rust-specific ideas at first useful contact instead of sending the reader to prerequisite material.
19. Keep chapter endings open-ended and forward-driving; avoid debrief summaries at chapter end.
20. Preserve print-first readability constraints: examples should fit narrow page width and break long Rust signatures predictably.
21. Use diagrams as `[mermaid]` blocks, never `[literal]` or bare ASCII art.

## Listing Grammar

- Use `[source,rust]` for Rust code.
- Use `[source,text]` for compiler diagnostics and benchmark/program output.
- Use `[source,shell]` for cargo and shell commands.
- Use `[source,toml]` for Cargo manifests.
- **Every concept-bearing listing MUST have a caption:** `.Caption text` directly above `[source,*]` block.
  - Format: `.Descriptive caption explaining what this listing demonstrates`
  - Completeness test: Does it answer "This listing shows...?"
  - Example: `.A zero-copy AddOrder: using borrowed slices instead of owned arrays`
- For intentionally broken code, place a `WARNING` block before the listing explaining the bug.

## Chapter Opening and Flow

- Between chapter heading and first section, keep preamble to 3-5 sentences.
- Open with callback, then immediate problem, then transition to first section.
- Keep financial domain context to one concise paragraph and place it exactly where first needed.
- Every section heading should pose an implicit reader question: "How do I do this?" or "Why did this fail?"
- Do not open sections with concept-summary. Open with a task, failure, artifact, or decision instead.
- Use explicit cross-references (`<<section-id>>`) instead of "as mentioned above" or "earlier in this chapter".

## Learning Model

- Every major section must answer an implicit reader question of the form "How do I do this?" or "Why did this attempt fail?"
- Do not open sections with concept-summary prose. Open with a task, a failure, an artifact, or a decision.
- Progressive discovery is mandatory: let the reader run code, hit an error, inspect output, or compare two versions before you explain the rule.
- Prefer active instructions such as "Try to compile this" or "Run this against the fixture" over explanatory throat-clearing.
- When a Rust-specific term appears for the first time, give the reader enough local meaning to continue. Do not require prior TRPL reading to follow the chapter.
- Bridge from familiar concepts explicitly: if the reader likely knows the idea from C++, Go, Java, or Python but not the Rust version, spend a paragraph on what carries over and what changes.
- If a section can be removed without changing what the reader does next, cut it.

## Prose Variation

- Mix short and long sentences.
- Mix single-paragraph sections with longer sections where the material earns it.
- Avoid repeating the same opener pattern across adjacent paragraphs.
- Avoid recycling the same high-level verbs (`learn`, `explore`, `understand`, `build`) when a more precise verb exists.
- Read section openings back-to-back; if they sound templated, rewrite them.

## Voice Anti-Patterns (check before submitting any prose)

These are the most common AI-generated prose failures in this codebase. Every chapter draft must pass this checklist:

- [ ] No credential projection — do not write "You've done X, you've seen Y" backstories for the reader.
- [ ] No ambient filler closers — cut any sentence ending a section that could be removed without loss ("Everything builds on this", "downstream of that", "feel necessary rather than academic").
- [ ] No idioms — "in your bones", "under the hood", "at the end of the day" are all cuts.
- [ ] No fragment-paragraph openers — "Twelve chapters. One project." is pitch deck energy, not prose.
- [ ] Dry observation has an implication, not advice — humor must land somewhere unexpected, not tell the reader to appreciate something.
- [ ] Do not tell the reader how the book will make them feel.
- [ ] Voice is "you", not "we" anywhere.
- [ ] The section teaches the reader how to act or decide, not merely what a concept is.
- [ ] At least one concrete action, artifact, error, or measurement appears before any extended explanation.
- [ ] First-contact Rust terms earn a local refresher instead of assuming prior Rust study.
- [ ] Adjacent paragraphs do not share the same cadence or sentence pattern by accident.
- [ ] The longest section in the chapter is one of the hardest or highest-payoff ideas.
