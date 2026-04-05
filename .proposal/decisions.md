# Editorial Decisions

These decisions are treated as standing instructions.

1. Every chapter opens with a callback to the previous exercise.
2. Domain context is one paragraph per chapter at most.
3. Every exercise runs against real Nasdaq sample data or deterministic fixtures derived from it.
4. No chapter debrief section; the next chapter opening is the debrief.
5. Testing enters in chapter 5 through error design, not a standalone testing chapter.
6. Threads vs async comparison is anchored in chapter 8.
7. Further reading belongs in repo resources, not chapter endings.
8. FFI chapter uses libz motivation for practical reader value.
9. No explicit How to use this book section.
10. Epilogue emphasizes PyO3 to Python as primary next step.
11. The book teaches how to do the work, not abstractly about Rust concepts.
12. The reader is the protagonist of every chapter; the prose follows their journey through action, friction, and resolution.
13. Progressive discovery is mandatory: examples, errors, outputs, and measurements precede extended explanation.
14. Prose must vary in cadence and structure enough to read authored rather than templated.
15. Page budget follows difficulty and payoff, not uniform section sizing.
16. Code explanation in chapter prose uses callouts under listings, not inline Rust comments, except short structural markers and required `// SAFETY:` notes.
17. Every concept-bearing listing in prose gets a listing caption and keeps source language tags accurate (`rust`, `text`, `shell`, `toml`).
18. Chapter prose uses explicit cross-references (`<<id>>`) instead of vague phrases like "as mentioned above".
19. Admonitions are sparse and intentional (roughly 2-3 per chapter max) and mapped by intent: `NOTE`, `TIP`, `WARNING`, `IMPORTANT`, `CAUTION`.
20. Production outputs target 6x9 print geometry with disciplined typography and code-block readability as first-class constraints.
21. Chapter endings should avoid recap/debrief summaries; the next chapter opening carries continuity.
