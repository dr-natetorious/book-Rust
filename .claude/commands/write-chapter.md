# write-chapter

Write chapter $ARGUMENTS end-to-end: source with tag markers, confidence tests, and chapter AsciiDoc prose.

---

## Step 1 — Read before writing

Read all of these before producing any output:

1. `.proposal/manifest.yaml` — find the entry where `id: $ARGUMENTS`. Note its `file`, `source_paths`, `test_paths`, and `status`.
2. `.proposal/chapters/$ARGUMENTS_*.md` if it exists — chapter-specific acceptance criteria.
3. `.proposal/decisions.md` — standing editorial decisions. Non-negotiable.
4. `.proposal/Overview.md` — the Table of Contents entry for this chapter.
5. `.proposal/Voice.md` — tone and narrative stance.
6. `.proposal/Humor.md` — one dry observation per chapter, placed at the exhale.
7. `.proposal/Artifacts.md` — rules for code placement and callouts.
8. `.proposal/SectionDensity.md` — pacing guidance where applicable.
9. The previous chapter `.adoc` — read its closing exercise for the callback opening.

---

## Step 2 — Write the Rust source

Create or update `src/chNN/`.

1. Wrap every prose-visible excerpt in AsciiDoc tag comments:
   ```rust
   // tag::concept_name[]
   // end::concept_name[]
   ```
2. Tags named after the concept, not the file section.
3. Intentionally broken examples go in a separate `_broken.rs` file.
4. No `unwrap()` in library code. Use named error types.
5. Deterministic: no `rand`, no time, no network.

---

## Step 3 — Write the confidence tests

Create or update `test/chNN/`.

1. Fixture-driven. Add fixtures under `test/fixtures/chNN_description.*`.
2. At minimum: one happy-path test for the exercise, one failure/error path test.
3. Wire into `test/integration_chNN.rs` and `Cargo.toml [[test]]` block.

---

## Step 4 — Write the chapter AsciiDoc

Update `chapters/NN_title.adoc` in full.

Structure (in order):

1. Level-2 heading matching manifest title.
2. Callback opening (2-4 sentences, "you" not "we", prior work creates new friction).
3. Domain context paragraph (one paragraph, max five sentences, placed when needed not front-loaded).
4. Chapter body following Overview.md structure: problem → friction → concept.
5. Code via include:: tag references only. No inline duplicates.
6. Exercise: one imperative task, fixture or real Nasdaq data.
7. One dry observation at the exhale after the hardest section.

Never include:
- "In this chapter we will learn..."
- A debrief or summary section.
- Inline code that duplicates a tagged src/ snippet.
- Further reading lists.

---

## Step 5 — Update the manifest

Set `status: written` and fill in `source_paths` and `test_paths` for chapter $ARGUMENTS in `.proposal/manifest.yaml`.

---

## Step 6 — Verify

1. `cargo test --test integration_chNN` passes.
2. Every `include::` resolves to an existing file and tag.
3. Exactly one domain-context paragraph.
4. Chapter opens with callback, ends with exercise, no debrief.

## Step 7 — Voice self-check

Read the chapter back against this list before finishing:

- [ ] No credential projection opener ("You've shipped X, you've debugged Y").
- [ ] No ambient filler closers ("Everything builds on this", "downstream of that", "feel necessary rather than academic").
- [ ] No idioms: cut "in your bones", "under the hood", "at the end of the day".
- [ ] No fragment-paragraph openers for false urgency.
- [ ] Dry observation has an unexpected implication, not advice.
- [ ] Never tells the reader how to feel about the material.
- [ ] "you" throughout, no "we".
- [ ] All diagrams are `[ditaa]` blocks, not `[literal]` or ASCII art.
- [ ] Every mentioned tool has a runnable command shown.
