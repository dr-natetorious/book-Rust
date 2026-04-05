# Artifact Placement and Usage

## The Governing Principle

Every artifact answers a question the prose just made the reader feel. Not decorates a section. Not signals effort. Answers a question.

If you can remove an artifact and the chapter argument still holds, the artifact was decorative. Cut it.

---

## The Freshness Test

Before placing any artifact, ask: does this convey something the prose cannot? Spatial relationships, data shape, system structure, process flow, direct tool output — these earn visuals. Information that's sequential belongs in prose. Information the reader already understands doesn't need a diagram.

---

## Artifact Types and Their Rules

### Code Snippets

The highest-frequency artifact. Requires the most discipline.

**Rules:**
1. Never show a snippet before the reader has a reason to want it
2. Annotate with comments that carry argument, not description
   - Dead weight: `// the compiler catches this`
   - Pays rent: `// this line transfers ownership — msg is gone after here`
3. Show broken code deliberately and often. What doesn't compile is frequently more instructive than what does
4. Compiler error output is part of the snippet — format it as such, annotated where useful

> "Try to compile this. Read the error before you read the next paragraph."

That instruction, placed before a broken snippet, gives the reader agency. They discover before they're told.

---

### Diagrams

Only when topology or flow is genuinely hard to hold in working memory. The test: can the reader construct this mental model from prose alone in under sixty seconds? If yes, no diagram needed.

**Legitimate uses in this book:**
- Full platform pipeline — once, in the preamble. Never again as a complete diagram
- Borrow checker aliasing decision tree — chapter 2
- Memory layout: stack frame with annotated lifetimes — chapter 2
- Iterator chain showing lazy evaluation — chapter 4
- Box/Rc/Arc ownership decision flowchart — chapter 7
- Async state machine expansion — chapter 10

**Not legitimate:**
A box labeled "Order Book" with arrows. The reader knows what a box is.

**Format rule:**
Diagrams must be written as `[mermaid]` blocks, not `[literal]` or ASCII art in code blocks. `asciidoctor-diagram` is installed in the container toolchain, and the image build provides `mmdc` plus the browser runtime Mermaid needs. A `[literal]` block renders as monospace text in all output formats. A `[mermaid]` block renders as a clean graphic in HTML, PDF, and EPUB. Use `[mermaid]`.

```asciidoc
[mermaid]
....
flowchart LR
   Source[Source] --> Result[Result]
....
```

---

### Tables

Use a table when you are comparing options along a decision axis and the reader needs to see all options at once to make a choice. The table replaces a paragraph of "on the other hand" comparisons, not a list of steps.

**Earns a table:** Box vs Rc vs Arc (ch. 7), monomorphization vs dynamic dispatch (ch. 6), unsafe superpowers with permitted operations and consequences (ch. 11).

**Does not earn a table:** sequential steps, summaries of preceding prose, reference material the reader will never revisit.

---

### Flamegraphs, Profiler Output, and Screenshots

Use when the visual is the evidence — when showing is faster and more precise than describing. The before/after flamegraph in chapter 8 is the clearest example: the gap between the two images is the chapter's conclusion. Don't narrate it; show it.

The same principle applies to any tool output that works as evidence: a `tokio-console` screenshot showing task state during a deadlock, a `cargo-asm` listing showing what the compiler actually produced. These are legitimate when prose would take a page to convey what the screenshot conveys in a glance.

The test: could a careful reader extract the same information from a prose description in under thirty seconds? If yes, prose is better. If no, use the visual.

---

### Tool Output (Miri, cargo-asm, tokio-console)

Raw tool output, lightly annotated, is its own artifact type. Treat it differently from code snippets — it's evidence, not instruction.

In chapter 11, the Miri output catching undefined behavior should be presented as the tool produced it, with minimal annotation. The reader interprets it. The prose comments afterward, briefly. This sequencing matters: tool output → reader interpretation → prose clarification, not the reverse.

---

### Side-by-Side Comparisons

Used twice in the book:

- **Chapter 3:** C++ union vs Rust enum. One page. No prose required — the structural difference is visible in the syntax. The prose that follows explains the consequence, not the diff.
- **Chapter 5:** Two error type designs, annotated with what the caller can and cannot do with each. This is the chapter's core artifact. A table won't work here — the argument lives in the code structure, not a comparison matrix.

---