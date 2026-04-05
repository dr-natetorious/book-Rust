---
applyTo: "chapters/**/*.adoc"
description: "Use when editing chapter prose in AsciiDoc, including callback openings and tagged source includes"
---

# Book Authoring Rules

## Core Principles

1. **Protagonist narrative** — The reader is the protagonist. Prose tracks their decisions, errors, measurements, and progress. Not the author explaining from a distance.

2. **Action first, explanation second** — Put the reader in motion early: action (try this), breakage (error), explanation (here's why). Progressive discovery is mandatory.

3. **One problem, one chapter** — Every chapter solves one concrete problem by building something that works. Code from chapter N is still running in chapter N+12.

4. **Callback structure** — Each chapter opens with a callback to the previous chapter's exercise. Each chapter ends with a forward-driving question that chapter N+1 answers. No chapter starts cold.

## Chapter Format

- **Heading:** `== Chapter N: Title` (number bound to title)
- **Preamble:** 3–5 sentences between heading and first section. State the problem. Transition to first task.
- **Domain context:** One paragraph max, placed exactly where first needed—not front-loaded. Never assume the reader needs background; they need just enough to see why the problem matters.
- **Sections:** Every section heading poses an implicit reader question: "How do I do this?" or "Why did this fail?" Open sections with a task, failure, artifact, or decision—never a concept summary.

## Code Listings

- **Language tags are mandatory:** `[source,rust]` for code, `[source,text]` for output/diagnostics, `[source,shell]` for commands, `[source,toml]` for manifests.
- **Captions are mandatory:** Every concept-bearing listing has a `.Caption text` line directly above the `[source,*]` block. The caption answers "This listing demonstrates...?"
  - Example: `.A zero-copy AddOrder: using borrowed slices instead of owned arrays`
- **Use include:: references:** Never duplicate source snippets inline. Reference tagged excerpts from src/chNN/ using `include::../src/chNN/file.rs[tag=tagname]`.
- **Intentionally broken code:** Place a `WARNING` block BEFORE the listing explaining the specific bug the reader will see.
- **Callout markers:** Use `<1>`, `<2>` in code for specific lines; explain below the listing. Avoid inline comments.
- **Cross-references:** Use `<<anchor-id>>` instead of "as mentioned above" or "earlier in this chapter".

## Admonitions (1–2 per chapter)

- `WARNING:` Safety violations, common pitfalls, irreversible consequences
- `NOTE:` Non-flow information essential for understanding (use sparingly)
- `TIP:` Optional patterns or shortcuts (use very sparingly)
- `IMPORTANT:` Critical preconditions or invariants
- `CAUTION:` Data loss or irreversible actions

Each admonition addresses a friction point specific to the chapter topic. Not generic advice.

## Prose Standards

- **Vary cadence:** Mix short and long sentences. Mix single-paragraph sections with longer ones. Avoid templated rhythm.
- **Budget by difficulty:** Hard ideas get room. Obvious steps move quickly. The longest section should be one of the hardest or highest-payoff concepts.
- **Assume engineering maturity, not Rust expertise:** Define Rust-specific terms locally (at first useful contact) instead of sending readers to prerequisite material. Bridge from C++/Go/Java/Python explicitly when the idea exists in those languages but works differently in Rust.
- **Teach how, not about:** The chapter teaches the reader how to act or decide, not what a concept is. Every major section answers "How do I do this?" or "Why did this fail?", not "What is a lifetime?"
- **Print-first constraints:** Rust signatures break across lines with `where` clauses. Code examples fit narrow pages. Use `[mermaid]` for diagrams, never ASCII art.

## Preservation Rules (Do Not Violate)

- **No silent abridgement:** Never replace long narrative sections with short summaries, checklist bullets, or compressed recap prose unless explicitly requested.
- **Task separation:** Build/debug/tooling requests are not permission to edit chapter narrative.
- **Depth parity checks:** For major rewrites, keep comparable section count and narrative depth; only shorten with explicit user direction.
- **Restore before proceed:** If prose gets condensed unintentionally, restore the affected chapter from the last known-good commit before any further edits.
- **Artifact-first mode:** If the user asks for immediate output (for example a current-state PDF), produce the artifact first and postpone prose/process refactors.

## Voice Anti-Patterns — ALWAYS CHECK

These are the most common failures that break the book's voice. Every draft must pass this filter:

- [ ] No credential projection ("You've already seen X, you know about Y")
- [ ] No filler closers ("This builds on everything", "Feel the power of ownership")
- [ ] No idioms ("in your bones", "under the hood", "at the end of the day")
- [ ] No fragment openers ("Twelve chapters. One project." = pitch deck, not prose)
- [ ] No emotional temperature declared ("feels uncomfortable", "you'll love", "good progress")
- [ ] No generic meta-commentary ("as you might expect", "downstream of")
- [ ] Voice is always "you" (reader as agent), never "we" (author and reader together)
- [ ] At least one concrete action, artifact, error, or measurement appears BEFORE any extended explanation
- [ ] Section openers don't repeat the same pattern (subject + verb + concept). Vary the rhythm.
- [ ] First-contact Rust terms get a local meaning, not a reference to TRPL or documentation
- [ ] No "you will feel" or "you will learn" or "this teaches you"—show, don't tell

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
