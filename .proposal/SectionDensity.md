# Section Density

## The Governing Principle

Density follows complexity and value, not word count targets. Some concepts take three paragraphs. Some take twelve. The discipline is not over-explaining simple things or under-explaining hard ones. If a section is long, it should be because the material earns it — not because the author spent a lot of time on it.

---

## Chapter-Level Guidance

### Chapters 1–2: Move Fast

These chapters should feel like a sprint. The reader is eager, the concepts are foundational, and the payoff is close. Short sections. Quick payoff.

Move semantics doesn't need four pages — it needs one tight page and a broken snippet that makes the point in two compiler errors. The one place that gets more room is the exercise section in chapter 1, because it's the first exercise in the book and the reader needs to understand what "exercise" means here. After that, the pattern is established and the exercise sections can trust the reader.

### Chapter 3: Slow Down on Purpose

Enums-as-tagged-unions is genuinely new to most readers coming from C++ or Java. The `match` exhaustiveness material needs room to breathe. More importantly, the "making illegal state unrepresentable" section earns its length — it's the first time the reader encounters a Rust design philosophy rather than a syntax rule. Don't rush it. This is where the language starts to feel like it has opinions, and the reader needs time to sit with that.

### Chapter 7: Don't Lose Them Here

This is where every Rust book loses readers. The section on lifetime elision rules should be *short* — the rule is simple, the fear around it is disproportionate. The section on `Rc` vs `Arc` tradeoffs needs room because the decision has real consequences and the reader needs to see the concrete problem first: three components, one shared state, what breaks if you get it wrong. Show the broken version. Walk through the compile error. Then show the fix. That three-beat sequence takes the space it takes.

### Chapter 8: Widest Variance of Any Chapter

Not all sections are equal here. "Establishing a baseline" is short — one page. The flamegraph walk-through is longer because you're teaching the reader to *read* an artifact, not just generate one, and that takes demonstration. The "one change, one measurement" discipline statement is two paragraphs. It's a principle, not a technique — it doesn't need more than that.

### Chapter 11: Earns the Most Space

The unsafe/FFI chapter is the most complex in the book and should use the page budget accordingly. The five unsafe superpowers section needs one concrete consequence per item — not just a name, not a lengthy explanation, but a single sentence that makes the risk tangible. The Miri section needs a full worked example: introduce the undefined behavior, run Miri, read the output, fix it. That three-beat sequence is the chapter's payoff and it shouldn't be compressed.

---

## The Density Test

Before finalizing any section's length, ask two questions:

1. Is this section long because the material is hard, or because I spent a lot of time on it? Rebalance if the latter.
2. Is the longest section in this chapter the hardest concept? If a simpler section is running long, cut it.

A page with one long paragraph, two medium ones, and a two-sentence closer reads like a human made decisions. A chapter where every section is roughly the same length reads like a template.