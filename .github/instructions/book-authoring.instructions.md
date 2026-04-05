---
applyTo: "chapters/**/*.adoc"
description: "Use when editing chapter prose in AsciiDoc, including callback openings and tagged source includes"
---

# Book Authoring Rules

1. Start each chapter with a callback to the previous chapter exercise.
2. Keep domain context to one concise paragraph.
3. Use include:: references to src tags for code.
4. Favor concrete engineering tradeoffs over broad theory.
5. Use diagrams as `[mermaid]` blocks, never `[literal]` or bare ASCII art.
6. Teach the reader how to do the work, not about the concept in the abstract.
7. Put the reader in motion early: action first, breakage second, explanation third.
8. Treat the reader as the protagonist of the chapter. The prose tracks their decisions, errors, and progress.
9. Vary cadence on purpose: do not let sections collapse into repetitive sentence length, paragraph length, or repeated phrasing.
10. Spend page budget according to difficulty and payoff. Hard ideas get room; obvious steps move quickly.
11. Assume engineering maturity, not prior Rust fluency. Define Rust-specific ideas at first useful contact instead of sending the reader to prerequisite material.

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
