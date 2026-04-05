# Style Audit - 2026-04-04

Scope: AsciiDoc chapter prose, theme/build configuration, and style-governance artifacts.

## Quantitative Findings

Per-chapter checks on `chapters/*.adoc` (excluding `_attributes.adoc` and `book.adoc`) show:

1. Callout markers (`<1>`, `<2>`, ...) in chapter prose: 0 across all chapters.
2. Listing captions (`.Caption` line before `[source,*]`): 0 across all chapters.
3. Admonition blocks (`NOTE`, `TIP`, `WARNING`, `IMPORTANT`, `CAUTION`): only 1 occurrence (in preamble).
4. Explicit cross-references (`<<id>>`): 0 occurrences.
5. Exercise heading presence: 1 per chapter (present across preamble and chapters 1-12).
6. Chapter heading style consistency:
   - `Chapter N:` prefix exists in chapters 1-3.
   - Prefix absent in chapters 4-12.

## Where We Are Breaking the New Guide

### 1) Listing grammar and annotation discipline

- No callout-driven explanations in chapter listings.
- No listing captions for concept-bearing code.
- One inline explanatory code comment appears in preamble compiler output snippet.

Impact:
- Technical explanations are less indexable and less publication-like.

### 2) Admonition discipline

- The book currently underuses admonitions relative to the intended style system.
- Risk and failure-prone transitions are not consistently marked with `WARNING` or `IMPORTANT`.

Impact:
- Important cautionary guidance is not visually distinct.

### 3) Cross-reference discipline

- No explicit internal cross-reference anchors/usages were detected.
- Narrative continuity relies on prose callbacks only.

Impact:
- Lower navigability in PDF/HTML and weaker structural cohesion for lookup reading.

### 4) Chapter title consistency

- Heading format is mixed between early and later chapters.

Impact:
- Inconsistent visual hierarchy and metadata tone.

### 5) Print theme depth

- `chapters/theme/pdf-theme.yml` remains minimal and does not yet encode full print grammar (running headers/footers, refined code block treatment, caption tuning, admonition visuals).

Impact:
- Current PDF renders correctly but does not yet project final production polish.

### 6) Web stylesheet depth

- `chapters/theme/web.css` establishes brand direction, but lacks deeper rules for blockquote/listing/admonition hierarchy consistency and long-form rhythm.

Impact:
- Web output is solid but not yet fully aligned with print-oriented house style.

## What Is Already Aligned

1. Tagged includes are widely used in chapter prose and avoid source duplication.
2. Chapter callback flow exists throughout chapters 2-12.
3. Exercise closures exist and are consistent.
4. Build pipeline already targets HTML/PDF/EPUB.
5. Container-first reproducible build path is available.

## Proposed Fix Plan (Everything)

### Phase 1 - Mechanical updates (fast, low-risk)

1. Standardize chapter heading format across chapters 1-12.
2. Add listing captions to each concept-bearing source block.
3. Add/normalize source block language tags for command/output/listing types.
4. Introduce 1-2 meaningful admonitions in each chapter where risk/friction is highest.
5. Add anchor IDs and explicit `<<id>>` cross-references for major recurring ideas.
6. Expand `chapters/theme/pdf-theme.yml` to include page geometry, typography hierarchy, code/admonition/caption/table styling, and running header/footer behavior.
7. Expand `chapters/theme/web.css` to mirror hierarchy (admonitions, blockquotes, code panes, and heading rhythm).

### Phase 2 - Editorial refactor (medium effort)

1. Convert explanatory inline commentary to callout explanations in chapter prose.
2. Ensure each chapter preamble remains within 3-5 sentences and transitions cleanly into the first section.
3. Tighten section-level one-idea-per-paragraph discipline where sections currently drift.
4. Normalize chapter endings to avoid recap language.

### Phase 3 - Validation and production pass

1. Validate include/tag integrity.
2. Run chapter and integration tests for touched chapters.
3. Build HTML/PDF/EPUB and inspect representative pages in print and grayscale.
4. Iterate on theme spacing, line-length pressure, and code block wrapping.

## Completion Criteria

1. Non-zero callouts and listing captions in every chapter with concept listings.
2. Admonition usage intentional and bounded.
3. Internal cross-reference graph in place for recurring concepts.
4. Unified chapter heading style.
5. PDF theme and web stylesheet match house style expectations.
6. Build/test pipeline remains green.
