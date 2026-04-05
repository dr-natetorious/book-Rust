---
applyTo: "chapters/**/*.adoc"
description: "Use when editing chapter prose in AsciiDoc, including callback openings and tagged source includes"
---

# Book Authoring Rules

1. Start each chapter with a callback to the previous chapter exercise.
2. Keep domain context to one concise paragraph.
3. Use include:: references to src tags for code.
4. Favor concrete engineering tradeoffs over broad theory.
5. Use diagrams as `[ditaa]` blocks, never `[literal]` or bare ASCII art.

## Voice Anti-Patterns (check before submitting any prose)

These are the most common AI-generated prose failures in this codebase. Every chapter draft must pass this checklist:

- [ ] No credential projection — do not write "You've done X, you've seen Y" backstories for the reader.
- [ ] No ambient filler closers — cut any sentence ending a section that could be removed without loss ("Everything builds on this", "downstream of that", "feel necessary rather than academic").
- [ ] No idioms — "in your bones", "under the hood", "at the end of the day" are all cuts.
- [ ] No fragment-paragraph openers — "Twelve chapters. One project." is pitch deck energy, not prose.
- [ ] Dry observation has an implication, not advice — humor must land somewhere unexpected, not tell the reader to appreciate something.
- [ ] Do not tell the reader how the book will make them feel.
- [ ] Voice is "you", not "we" anywhere.
