---
name: write-chapter
description: "Write a complete chapter: prose, source with tag markers, confidence tests, and manifest update"
---

Write chapter **${input:chapter_id}** end-to-end. Follow every constraint below exactly.

---

## Step 1 — Read before writing

Read all of these before producing any output:

1. `.proposal/manifest.yaml` — find the entry where `id: ${input:chapter_id}`. Note its `file`, `source_paths`, `test_paths`, and `status`.
2. `.proposal/chapters/${input:chapter_id}_*.md` if it exists — chapter-specific acceptance criteria and source requirements.
3. `.proposal/decisions.md` — standing editorial decisions, non-negotiable.
4. `.proposal/Overview.md` — the Table of Contents entry for this chapter (title, concepts, exercise, page guidance).
5. `.proposal/Voice.md` — tone and narrative stance.
6. `.proposal/Humor.md` — one dry observation per chapter, placed at the exhale.
7. `.proposal/Artifacts.md` — rules for code snippets, callouts, and when to use them.
8. `.proposal/SectionDensity.md` — per-chapter pacing guidance where available.
9. The previous chapter's `.adoc` file — read its closing exercise so you can write the callback opening.
10. While reading, extract the chapter's reader journey in one line: `task -> failure/friction -> discovery -> working result`.

---

## Step 2 — Write the Rust source

Create or update `src/chNN/` where `NN` is the zero-padded chapter id.

Rules:
1. Every code excerpt that will appear in prose must be wrapped with AsciiDoc tag comments:
   ```rust
   // tag::your_tag_name[]
   // your code
   // end::your_tag_name[]
   ```
2. Tags must be named after the concept they represent, not the line number or file section.
3. Code must compile. If it is intentionally broken for the reader to try, put it in a separate file suffixed `_broken.rs` and mark it prominently with a comment.
4. Use explicit, named error types. No `unwrap()` in library code.
5. Keep examples deterministic: no `rand`, no system time, no network calls.
6. Prefer source layouts that let the reader run something early and then extend it, rather than dumping the final design all at once.

---

## Step 3 — Write the confidence tests

Create or update `test/chNN/`.

Rules:
1. Write tests that cover the concepts taught, not every function permutation.
2. Use deterministic fixtures from `test/fixtures/`. Add new fixtures if needed — name them `chNN_description.hex` or similar.
3. Include at least:
   - One test that validates the happy path for the chapter exercise.
   - One test that validates the primary failure / error path.
4. Wire the tests into `test/integration_chNN.rs` and add a `[[test]]` entry in `Cargo.toml` if not already present.

---

## Step 4 — Write the chapter AsciiDoc

Write `chapters/NN_title.adoc`. The file must already exist as a stub — update it in full.

### Structure (in order)

1. **Level-2 section heading** — matches the manifest title exactly.

2. **Callback opening** — 2–4 sentences. Reference the specific artifact the reader built in the previous chapter.
   - Use "you", not "we".
   - The prior chapter's work becomes the source of the new problem — not a summary.
   - Do NOT write: "In the last chapter, we learned about X."

3. **Domain context paragraph** — exactly one paragraph. Maximum five sentences.
   - Placed right when the reader needs it, not at the top.
   - Never front-loaded.

4. **Chapter body** — sections follow the structure in `.proposal/Overview.md` for this chapter.
   - Problem first. Friction second. Concept third. Never concept first.
   - The prose answers "how do I do this?" and "why did that fail?" more often than "what is X?"
   - Each major section needs a visible reader action: run code, inspect output, read an error, compare two implementations, or make a design choice.
   - When a Rust-specific idea appears for the first time, add a concise bridge from concepts the reader likely knows already: pointers, slices, tagged unions, interfaces, ownership of heap data, or thread-safety rules.
   - Code included via tag references only:
     ```asciidoc
     [source,rust]
     ----
     include::../src/chNN/filename.rs[tag=your_tag_name]
     ----
     ```
   - Broken-code exercises use the same include pattern with a note before them:
     ```asciidoc
     Try to compile this. Read the error before you read the next paragraph.
     ```
   - Compiler error output shown as a listing block, lightly annotated.
   - Comments inside code blocks carry argument, not description.

5. **Exercise** — one concrete task, runs against fixture or real Nasdaq data.
   - Stated as an imperative: "Run the parser against the fixture and print every Add Order record."
   - No debrief after the exercise. The next chapter's callback is the debrief.
   - The exercise must feel like the natural next move after the chapter body, not a detached homework problem.

6. **One dry observation** — placed at the exhale after the hardest section.
   - Technically accurate, slightly sinister, one or two sentences.
   - Never mid-concept.
   - See `.proposal/Humor.md` for register and examples.

### What never appears in a chapter

- "In this chapter we will learn..."
- A debrief or summary section at the end.
- Inline code duplicated from `src/` that already has a tag.
- Further reading lists.
- "How to use this section" framing.

---

## Step 5 — Update the manifest

In `.proposal/manifest.yaml`, find the entry for chapter `${input:chapter_id}` and:
1. Set `status: written`.
2. Fill in all `source_paths` and `test_paths` for the files you created.

---

## Step 6 — Verify

1. Confirm `cargo test --test integration_chNN` passes (or report exactly what fails and why).
2. Confirm every `include::` in the chapter resolves to an existing file and tag.
3. Confirm the chapter has exactly one domain-context paragraph.
4. Confirm the chapter opens with a callback and ends with an exercise — no debrief.
5. Confirm the reader journey is visible on the page: task, friction, discovery, working result.
6. Confirm the longest section is one of the chapter's hardest or most valuable ideas.

## Step 7 — Voice self-check (read the prose back against this list)

This step is not optional. Read the chapter you just wrote and confirm each item:

- [ ] No credential projection: the opener does not invent a backstory for the reader ("You've shipped X, you've debugged Y").
- [ ] No ambient filler closers: no sentence ends a section whose only job is to summarize or encourage ("Everything from here builds on it", "downstream of that", "feel necessary rather than academic").
- [ ] No idioms standing in for concrete language: cut "in your bones", "under the hood", "at the end of the day".
- [ ] No fragment-paragraph openers used for false urgency ("Twelve chapters. One project.").
- [ ] The dry observation has an unexpected implication — it does not give advice or tell the reader to appreciate something.
- [ ] The chapter never tells the reader how to feel about the book or the material.
- [ ] Voice is "you" throughout. No "we".
- [ ] All diagrams are `[mermaid]` blocks, not `[literal]` or ASCII art in source blocks.
- [ ] Any tool or command mentioned in prose is shown with a runnable invocation or explicitly noted that one is not available.
- [ ] The chapter teaches the reader how to do the work, not merely what the topic means.
- [ ] Every major section contains a concrete action, artifact, error, output, or decision before extended explanation.
- [ ] Rust-specific ideas get a short first-contact refresher instead of assuming prior Rust reading.
- [ ] Sentence and paragraph rhythm vary across adjacent sections; the prose does not read templated.
- [ ] Important or difficult ideas are given more real estate than straightforward setup.
