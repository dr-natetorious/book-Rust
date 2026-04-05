# AsciiDoc Style & Structure Guide
## *Rust for Engineers Who Ship Things*
### How to make a self-published technical book look like a real O'Reilly publication

---

## The core problem with self-published technical books

Most self-published books look cheap because of five compounding failures:

1. **Wrong typeface choices** — system fonts, no contrast between code and prose
2. **No visual hierarchy** — everything looks like one level of importance
3. **Inconsistent spacing** — content crammed together or floating randomly
4. **Undisciplined code blocks** — inconsistent syntax highlighting, wrong widths
5. **No production pipeline** — raw AsciiDoc converted directly to PDF with default settings

O'Reilly books look like O'Reilly books because they have a typesetter, a house style, and a production team. You can replicate 90% of that with the right toolchain and the discipline to never deviate from the rules below.

---

## Toolchain

The correct stack for "web + Amazon KDP" publication using AsciiDoc:

```
AsciiDoc source (.adoc)
    → Asciidoctor (Ruby gem) for HTML web version
    → Asciidoctor PDF (with custom theme YAML) for print/KDP
    → Asciidoctor EPUB3 for Kindle distribution
```

Install:

```bash
gem install asciidoctor
gem install asciidoctor-pdf
gem install asciidoctor-epub3
gem install rouge           # syntax highlighting
gem install asciidoctor-diagram  # if you use diagram blocks
```

**Why not Pandoc?** Pandoc's PDF output via LaTeX looks academic, not technical-professional. Asciidoctor PDF gives you direct control over the theme without fighting LaTeX.

**Why not Sphinx or Hugo?** Single-source publishing (one .adoc → HTML + PDF + EPUB) is the goal. AsciiDoc was designed for this. Sphinx is Python-documentation culture; Hugo is web-first.

---

## Typography: The single biggest differentiator

O'Reilly's house fonts for their current generation of books:

- **Body text:** UbuntuMono for code; Noto Serif or a close relative for body prose in some series; **many newer O'Reilly books use a clean humanist sans** (similar to Source Serif Pro)
- **Code:** Always a monospace with clear differentiation between `0O` and `1Ill`

For your book, use this free stack (all available from Google Fonts or as OFL-licensed downloads):

| Role | Font | Why |
|---|---|---|
| Body prose | **Source Serif 4** | Neutral, editorial, designed for long reading, free |
| Headings | **Source Sans 3** or **IBM Plex Sans** | Clean, technical, pairs well with Source Serif |
| Code blocks | **JetBrains Mono** or **Iosevka** | Wide character differentiation, Rust-community standard |
| Captions | Source Sans 3, lighter weight | Distinct from body without needing a third family |

**What NOT to use:**
- Times New Roman, Georgia (academic, not technical)
- Arial, Helvetica, system-ui (unset default look)
- Fira Code alone without a matching body font (mismatch)
- Any font at a size below 10pt in print or 14px on web

### Type sizes (print, for KDP 6×9 trim)

```yaml
# In your Asciidoctor PDF theme:
base:
  font_size: 10.5        # body text
  line_height: 1.5       # generous — O'Reilly uses 1.4–1.6
heading:
  h1_font_size: 24
  h2_font_size: 18
  h3_font_size: 13
  h4_font_size: 11       # same as body but bold
code:
  font_size: 9.5         # slightly smaller than body is correct
caption:
  font_size: 9
```

---

## Page geometry (for KDP 6×9 trim)

Amazon KDP standard technical book trim: **6 × 9 inches**

```yaml
page:
  size: [6in, 9in]
  margin: [0.875in, 0.75in, 0.875in, 0.875in]  # top, right, bottom, left
  # Left margin slightly wider for gutter on print
```

The margins above match O'Reilly's approximate interior margins. Never use symmetric margins on a bound book — the gutter eats into your left margin.

**Text area:** Approximately 4.5 × 7.25 inches. This is your working canvas.

**For code blocks:** At 9.5pt JetBrains Mono, you get approximately **70 characters per line** before wrapping. Plan your code examples around this. Long Rust trait signatures will wrap — design them to wrap gracefully or break them across explicit lines in the source.

---

## The Asciidoctor PDF theme file

Create `themes/oreilly-style.yml`. This is the single most important file in your production pipeline. Every visual decision flows through here.

```yaml
# themes/oreilly-style.yml

extends: default

page:
  size: [6in, 9in]
  margin: [0.875in, 0.75in, 0.875in, 0.875in]

base:
  font_family: Source Serif 4
  font_size: 10.5
  line_height: 1.5
  font_color: 1a1a1a        # near-black, not pure black (easier on eyes)

font:
  catalog:
    Source Serif 4:
      normal: SourceSerif4-Regular.ttf
      bold: SourceSerif4-Bold.ttf
      italic: SourceSerif4-Italic.ttf
      bold_italic: SourceSerif4-BoldItalic.ttf
    Source Sans 3:
      normal: SourceSans3-Regular.ttf
      bold: SourceSans3-Bold.ttf
      italic: SourceSans3-Italic.ttf
    JetBrains Mono:
      normal: JetBrainsMono-Regular.ttf
      bold: JetBrainsMono-Bold.ttf
      italic: JetBrainsMono-Italic.ttf

heading:
  font_family: Source Sans 3
  font_color: 1a1a1a
  font_style: bold
  h1_font_size: 24
  h2_font_size: 18
  h3_font_size: 13
  h4_font_size: 10.5
  h1_margin_top: 0
  h1_margin_bottom: 18
  h2_margin_top: 24
  h2_margin_bottom: 10
  h3_margin_top: 18
  h3_margin_bottom: 6

# Chapter titles (mapped to part titles and chapter titles in AsciiDoc)
title_page:
  font_family: Source Sans 3
  font_size: 28
  font_color: 1a1a1a
  title_top: 30%            # vertical centering, like O'Reilly

# Running headers
header:
  font_family: Source Sans 3
  font_size: 8
  font_color: 666666
  height: 0.5in
  border_width: 0
  border_color: cccccc
  recto_content:
    right: '{section-title}'   # right-hand pages: current section
  verso_content:
    left: '{chapter-title}'    # left-hand pages: current chapter

# Page numbers
footer:
  font_family: Source Sans 3
  font_size: 9
  font_color: 666666
  height: 0.375in
  recto_content:
    right: '{page-number}'
  verso_content:
    left: '{page-number}'

# Code blocks — the most important styling decision in a technical book
code:
  font_family: JetBrains Mono
  font_size: 9.5
  line_height: 1.4
  background_color: f5f5f0   # warm off-white, not pure white or gray
  border_color: d0cfc8
  border_radius: 3
  border_width: 0.5
  padding: [8, 10, 8, 10]
  font_color: 1a1a1a

# Inline code (backtick spans in prose)
codespan:
  font_family: JetBrains Mono
  font_size: 0.9em           # slightly smaller than surrounding text
  background_color: f0efe8
  font_color: 1a1a1a

# Admonition blocks (NOTE, TIP, WARNING, IMPORTANT, CAUTION)
admonition:
  font_family: Source Sans 3
  column_rule_width: 0
  padding: [8, 12, 8, 12]
  label:
    font_style: bold
    font_size: 8
    text_transform: uppercase
  note:
    background_color: eef4fb
    border_color: 5b8db8
    border_width: [0, 0, 0, 3]   # left border only, like O'Reilly
  tip:
    background_color: eef6ee
    border_color: 5b9b5b
    border_width: [0, 0, 0, 3]
  warning:
    background_color: fff8ee
    border_color: b8892b
    border_width: [0, 0, 0, 3]
  important:
    background_color: fef0f0
    border_color: b85b5b
    border_width: [0, 0, 0, 3]

# Tables
table:
  font_size: 9.5
  head:
    font_family: Source Sans 3
    font_style: bold
    font_size: 9
    background_color: e8e8e2
    font_color: 1a1a1a
  body:
    stripe_background_color: f5f5f0
  border_width: 0.5
  border_color: cccccc
  cell_padding: [4, 8, 4, 8]

# Captions (figure/table captions)
caption:
  font_family: Source Sans 3
  font_style: italic
  font_size: 9
  font_color: 555555
  text_align: left
  margin_bottom: 6

# Block quotes (used sparingly — you use these for callout prose)
quote:
  font_style: italic
  font_color: 444444
  border_color: cccccc
  border_width: [0, 0, 0, 3]
  padding: [6, 12, 6, 16]

# Lists
list:
  item_spacing: 4
ulist:
  marker:
    disc:
      font_color: 888888

# Chapter preamble / lead paragraph styling
# (AsciiDoc doesn't have a built-in "lead" role, but you can assign [.lead])
role:
  lead:
    font_size: 12
    line_height: 1.6
    font_color: 333333
```

---

## AsciiDoc source structure

### Project layout

```
book/
├── book.adoc               ← master file, includes everything
├── themes/
│   └── oreilly-style.yml
├── fonts/                  ← embed your TTFs here
├── images/
│   ├── ch01/
│   ├── ch02/
│   └── ...
├── chapters/
│   ├── preamble.adoc
│   ├── ch01.adoc
│   ├── ch02.adoc
│   └── ...
└── scripts/                ← build and validation entry points
```

### Master file (book.adoc)

```asciidoc
= Rust for Engineers Who Ship Things
Your Name
:doctype: book
:toc: left
:toc-title: Table of Contents
:toclevels: 2
:sectnums:
:sectnumlevels: 2
:chapter-label: Chapter
:part-label: Part
:source-highlighter: rouge
:rouge-style: github        ← or pastie, monokai — test which looks best in PDF
:listing-caption: Listing
:figure-caption: Figure
:table-caption: Table
:icons: font
:icon-set: fas
:imagesdir: images
:pdf-themesdir: themes
:pdf-theme: oreilly-style

\include::chapters/preamble.adoc[]

= Part One: Getting Data In

\include::chapters/ch01.adoc[]
\include::chapters/ch02.adoc[]

= Part Two: Modeling State

\include::chapters/ch03.adoc[]
\include::chapters/ch04.adoc[]

...
```

### Chapter file structure

Every chapter follows this exact AsciiDoc pattern:

```asciidoc
[[ch01]]
== Parse a Binary Market Feed from Raw Bytes

// Opening callback — reference the previous exercise or preamble
// For ch01 this is the setup; for ch02+ this opens with what the reader
// just built and what breaks next.

[.lead]
Orders arrive as raw bytes — no JSON, no schema, no ceremony.
Before you can do anything useful with market data, you have to
read it. This chapter is about that, and why Rust makes it harder
than you expect.

=== How a market feed works

// The one-paragraph domain context, placed exactly here.
// Never more than this. Never front-loaded.

The Nasdaq ITCH 5.0 protocol encodes market events as fixed-width
binary messages...

=== Reading binary: big-endian, fixed-width, no ceremony

// Technical content begins. Code examples use callouts.

[source,rust]
----
use std::io::{self, BufRead};

fn parse_add_order(buf: &[u8]) -> AddOrder { // <1>
    let price = u64::from_be_bytes(buf[10..18].try_into().unwrap()); // <2>
    // ...
}
----
<1> The function signature commits to zero allocation — `&[u8]` is a borrow.
<2> Nasdaq ITCH uses big-endian encoding throughout. `from_be_bytes` is zero-copy.

// IMPORTANT: Callout numbers (<1>, <2>) are O'Reilly's preferred way to
// annotate code. Use them instead of inline comments for anything
// that needs more than ~5 words of explanation.

=== Ownership and the drop model

...

=== Move semantics

...

[[ch01-exercise]]
=== Exercise: Print every Add Order

[NOTE]
====
*What you need:* The Jan 30 2020 Nasdaq sample file downloaded
from `emi.nasdaq.com/ITCH/`. See the Preamble if you haven't done this.
====

// Exercise text

[TIP]
====
Run with `cargo run --release`. Debug builds are fine for compilation
but will be 10–20× slower against a 5GB file.
====
```

---

## The visual grammar of O'Reilly technical books

These are the specific visual decisions that separate "looks professional" from "looks like someone ran Pandoc."

### 1. Code blocks with callouts, not inline comments

This is the single most visible signal of a professional technical book. O'Reilly books almost never explain code inside the code. They use numbered callouts in the listing, then explain each one below.

```asciidoc
[source,rust]
----
fn parse_message<'a>(buf: &'a [u8]) -> Result<Message<'a>, ParseError> { // <1>
    match buf[0] {
        b'A' => parse_add_order(&buf[1..]),     // <2>
        b'X' => parse_cancel(&buf[1..]),
        b'D' => parse_delete(&buf[1..]),
        tag  => Err(ParseError::UnknownTag(tag)),
    }
}
----
<1> The lifetime `'a` binds the returned `Message` to the input buffer —
    no allocation, no copy.
<2> Single-byte tags make the dispatch table a single branch prediction miss,
    worst case.
```

**Never do this:**

```rust
fn parse_message<'a>(buf: &'a [u8]) -> Result<Message<'a>, ParseError> {
    // The lifetime 'a binds the returned Message to the input buffer
    match buf[0] {
        b'A' => parse_add_order(&buf[1..]),  // Add Order message
```

The first version teaches. The second version annotates. These are different.

### 2. Admonition blocks used sparingly and correctly

O'Reilly uses five admonition types. Use them precisely:

| Type | When to use |
|---|---|
| `NOTE` | Information the reader needs but that breaks the flow |
| `TIP` | Optional optimization or shortcut |
| `WARNING` | Will cause a real problem if ignored |
| `IMPORTANT` | Critical, but not dangerous |
| `CAUTION` | Potential data loss or irreversible action |

In a Rust book: `WARNING` for "this compiles but introduces UB," `NOTE` for toolchain-specific behavior, `TIP` for cargo flags.

**Use at most 2–3 admonition blocks per chapter.** If every other paragraph is a NOTE, you've lost the signal.

### 3. Figure captions below, table captions above

This is a typographic convention so standardized it's invisible when correct and jarring when wrong:

```asciidoc
// Table: caption ABOVE
.Message types and their allocation behavior
[cols="2,1,3", options="header"]
|===
| Message type | Allocation | Notes
| Add Order    | Zero       | Borrows from input buffer
| Execute      | Zero       | Reference to existing order
| Replace      | One        | New order, old order cancelled
|===

// Figure: caption BELOW
image::ch01/itch-message-layout.png[ITCH message wire format]
.Figure 1-1. The ITCH 5.0 Add Order message: 36 bytes, big-endian, fixed-width.
```

### 4. Consistent listing captions

Every non-trivial code listing gets a caption:

```asciidoc
.The feed handler's main parse loop
[source,rust,linenums]
----
...
----
```

This produces "Listing 1-1" in the output, which you can cross-reference. O'Reilly uses this consistently. It makes a book feel indexed and navigable.

### 5. Cross-references, not forward references

O'Reilly books are heavily cross-referenced. In AsciiDoc:

```asciidoc
// In ch01:
[[zero-copy-slicing]]
=== Zero-copy slicing

// In ch07, when you refer back:
As established in <<zero-copy-slicing>>, borrowing from the input
buffer is how we avoid allocation on the hot path.
```

This compiles to "see Section 2.2" in PDF and a hyperlink in HTML. It signals that the book was designed as a coherent system, not assembled from independent chapters.

### 6. The chapter-opening structure

Every O'Reilly chapter has an implicit three-part opening:

1. **A short orienting statement** — one or two sentences. What this chapter is about.
2. **The problem to be solved** — what the reader doesn't have yet.
3. **A transition into the first section** — often just a sentence.

In AsciiDoc, the text between the chapter heading and the first section heading is the "chapter preamble." Make it count. Keep it short — 3–5 sentences.

This is different from a "chapter objectives" box. Never use objectives boxes. They belong in textbooks, not engineering books. The preamble does the work implicitly.

### 7. Sidebar blocks for genuinely optional content

O'Reilly uses sidebars for content that is interesting but not essential to the main flow. Use them extremely sparingly — one per chapter maximum, and only when the content genuinely is optional:

```asciidoc
.Why big-endian?
****
Network byte order is big-endian for historical reasons going back
to the 1980 Xerox standard. Nasdaq inherited this convention from
the exchange infrastructure it was built on. Your CPU is almost
certainly little-endian, which is why `from_be_bytes` exists.
****
```

---

## Syntax highlighting

For a Rust book, Rouge's `github` style is clean and works well in print. For production, you want to verify the color contrast holds in grayscale (for readers who print or use Kindle's grayscale mode).

Recommended theme for this book's color palette: **`pastie`** — muted, professional, holds up in grayscale, doesn't fight with the page.

To test: generate one chapter, print to grayscale PDF, check that keyword vs. identifier distinction still reads.

---

## Script-first build workflow

This repository uses script entry points, not a Makefile-first workflow.

Use:

```bash
# Unix/macOS/WSL
bash scripts/build-book.sh
```

```bat
REM Windows
scripts\build-book.bat
```

Both wrappers target HTML, PDF, and EPUB outputs and preserve container reproducibility.

---

## Amazon KDP specific requirements

KDP has specific PDF requirements for their printing process. Your PDF needs to meet these exactly or it will fail their preflight:

- **Trim size:** 6 × 9 inches (set this in your theme, confirmed above)
- **Bleed:** 0.125 inches on all sides if you have full-bleed elements (you probably don't — skip this)
- **Margins:** Minimum 0.25 inches; recommended 0.75 inches (your theme exceeds this)
- **Resolution:** Images must be 300 DPI minimum for print; 72 DPI is fine for web/EPUB
- **Fonts:** Must be embedded — Asciidoctor PDF embeds all fonts automatically
- **Color mode:** KDP black-and-white interior requires grayscale images. Your code examples are text, which is fine. Any diagrams with color will be converted to grayscale — design for grayscale from the start.
- **ISBN:** Required for print; KDP will assign a free one if you don't purchase your own

**Image preparation for print:**

```bash
# Convert all PNGs to 300 DPI grayscale for print
for img in images/**/*.png; do
  convert "$img" -density 300 -colorspace Gray "${img%.png}-print.png"
done
```

---

## Web output (HTML)

The Asciidoctor HTML output with a custom stylesheet can look excellent. The default `asciidoctor.css` is serviceable but not polished.

For web, extend with a CSS file:

```css
/* custom.css — loaded via :stylesheet: custom.css in book.adoc */

:root {
  --body-font: 'Source Serif 4', Georgia, serif;
  --heading-font: 'Source Sans 3', system-ui, sans-serif;
  --code-font: 'JetBrains Mono', 'Fira Code', monospace;
  --text-color: #1a1a1a;
  --bg-color: #ffffff;
  --code-bg: #f5f5f0;
  --accent: #c0392b;        /* O'Reilly uses a warm red accent */
  --max-width: 780px;
}

body {
  font-family: var(--body-font);
  font-size: 18px;
  line-height: 1.6;
  color: var(--text-color);
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 2rem;
}

h1, h2, h3, h4 {
  font-family: var(--heading-font);
  font-weight: 600;
  line-height: 1.25;
}

pre, code {
  font-family: var(--code-font);
  font-size: 0.875em;
}

pre {
  background: var(--code-bg);
  border-left: 3px solid var(--accent);
  padding: 1rem 1.25rem;
  overflow-x: auto;
  border-radius: 0 4px 4px 0;
}

/* Admonition blocks */
.admonitionblock {
  border-left: 3px solid;
  padding: 0.75rem 1rem;
  margin: 1.5rem 0;
  border-radius: 0 4px 4px 0;
}
.admonitionblock.note  { border-color: #5b8db8; background: #eef4fb; }
.admonitionblock.tip   { border-color: #5b9b5b; background: #eef6ee; }
.admonitionblock.warn  { border-color: #b8892b; background: #fff8ee; }
```

---

## The ten rules you cannot break

These are the decisions that separate a book that looks like a real publication from one that looks like a Word document:

1. **Every code listing that explains a concept gets callout numbers.** No inline comments explaining Rust semantics. Those belong in callouts.

2. **Chapter numbers are spelled out in titles.** "Chapter One" in prose; `== Chapter 1: ...` is never written — the `== ` heading and part structure provide the number automatically.

3. **No bold in the middle of a sentence.** Bold is for terms being defined (on first use only), not for emphasis. Use italics for emphasis.

4. **Terms are italicized on first introduction, then used plainly.** "_Move semantics_ is the rule that..." After that, just "move semantics."

5. **Code font (backtick) for any identifier, function name, type name, or flag that appears in prose.** "`Result`, `Option`, and `?` form Rust's error handling trio."

6. **One idea per paragraph.** O'Reilly prose is dense but structured. A paragraph in a technical book is a unit of argument, not a container for related sentences.

7. **Figures and tables are numbered and captioned.** Always. "Figure 1-1" and "Table 3-2" are standard. Readers cite them in their notes.

8. **No "as we said earlier" or "as mentioned above."** Use cross-references. `<<zero-copy-slicing>>` not "earlier in this chapter."

9. **Admonition blocks cannot contain code listings.** If you need to say "NOTE: see this code," restructure — put the code in the main flow, then the NOTE after it.

10. **Every chapter ends in the middle of a thought.** Not with a summary, not with "in the next chapter." The next chapter's opening is the summary. This is your own editorial standing instruction — enforce it here too.

---

## What the reader actually notices

The reader doesn't consciously notice typography. What they notice is:

- *"This code is easy to read"* — that's JetBrains Mono at the right size with callouts instead of comments
- *"I never get lost"* — that's running headers and consistent cross-references
- *"This feels authoritative"* — that's Source Serif 4 at 10.5pt on 6×9 with real margins
- *"I can tell what's important"* — that's disciplined use of admonitions and bold

If those four things are true, the book looks like it cost $49.99. If any of them fail, it looks self-published in the pejorative sense.

The goal isn't to deceive anyone. The goal is to ensure the production quality doesn't create friction between the reader and your content — which, in this case, is genuinely excellent.

---

# Rust Style Guide

Rust code in print has specific failure modes that don't exist in other languages. The rules below apply to every listing in the book.

---

## The line width problem is worse in Rust than almost any language

Rust's type signatures are verbose by design. At 70 characters per line on a 6×9 page, these break badly:

```rust
// This is 87 characters. It will wrap mid-token in PDF.
pub fn process_message<'a, T: MessageHandler + Send + Sync>(buf: &'a [u8], handler: &T) -> Result<Message<'a>, ParseError> {
```

You need a house style for breaking long signatures **before typesetting forces a break**. Decide this now and apply it consistently:

```rust
pub fn process_message<'a, T>(
    buf: &'a [u8],
    handler: &T,
) -> Result<Message<'a>, ParseError>
where
    T: MessageHandler + Send + Sync,
{
```

The `where` clause style is preferable to inline bounds in print for exactly this reason — it breaks predictably and reads well in narrow columns. Make it your default for anything with more than one bound.

---

## Compiler errors are content

Your book explicitly uses compiler errors as teaching moments. They need a distinct visual treatment — they're not code the reader writes, they're output they read.

In AsciiDoc:

```asciidoc
[source,text]
----
error[E0502]: cannot borrow `order_book` as mutable
              because it is also borrowed as immutable
  --> src/main.rs:42:5
   |
39 |     let snapshot = order_book.get_snapshot();
   |                    ---------- immutable borrow occurs here
42 |     order_book.update(msg);
   |     ^^^^^^^^^^ mutable borrow occurs here
43 |     println!("{}", snapshot.best_bid);
   |                    -------- immutable borrow later used here
----
```

Use `[source,text]` not `[source,rust]` for compiler output — you don't want syntax highlighting fighting the error's own ASCII annotations. The compiler's output is already structured; leave it alone.

For exercises where you ask the reader to introduce a bug deliberately, flag the listing before it appears:

```asciidoc
[WARNING]
====
*Deliberately broken code.* The following listing will not compile.
That's the point. Read the error before continuing.
====
```

---

## Lifetime annotations in print

Lifetimes are visually noisy and readers' eyes slide off them. Two rules:

**Introduce each lifetime annotation once with a callout, then use it without comment.** Don't re-explain `'a` every time it appears after chapter 7.

**Never elide lifetimes in teaching examples, even where the compiler allows it.** If the concept being taught involves lifetimes, write them explicitly. Elision in production code is fine; in a teaching context it hides the mechanism.

```rust
// Wrong for teaching — lifetime is invisible
fn get_best_bid(book: &OrderBook) -> &Price {

// Right for teaching — makes the connection explicit
fn get_best_bid<'a>(book: &'a OrderBook) -> &'a Price {
```

---

## Traits, generics, and `impl` blocks create vertical sprawl

A trait with several method signatures plus a blanket impl can run 40–60 lines without containing much information. In print this reads as wall-of-code.

Show the interface separately from the implementation, with a clear break in the prose:

```asciidoc
First, the trait definition — what callers see:

[source,rust]
----
pub trait RiskCheck {
    fn check(&self, order: &Order) -> RiskResult;
    fn name(&self) -> &str;
}
----

Then a concrete implementation — one of several the reader will write:

[source,rust]
----
pub struct PositionLimit {
    max_notional: u64,
}

impl RiskCheck for PositionLimit {
    fn check(&self, order: &Order) -> RiskResult {
        // ...
    }

    fn name(&self) -> &str {
        "position_limit"
    }
}
----
```

Never show a trait definition and its implementation in a single listing unless the relationship between them is the point being taught.

---

## `//` comment discipline in listings

You have two annotation tools: callout numbers and inline comments. They are not interchangeable.

| Use callouts for | Use inline comments for |
|---|---|
| Explaining Rust mechanics | Marking a conceptually distinct block |
| Anything requiring > 5 words | `// Hot path` or `// Safety: see invariant below` |
| Cross-references to text | Intentionally idiomatic `// SAFETY:` docs |
| Surprising or non-obvious choices | Section breaks in longer listings |

One specific case: `unsafe` blocks. Always comment them with `// SAFETY:` — this is idiomatic Rust, your reader expects it, and it models the discipline you want them to internalize:

```rust
// SAFETY: buf is guaranteed to be at least 36 bytes by the
// ITCH framing layer before this function is called.
let price = unsafe {
    u64::from_be_bytes_unchecked(buf.get_unchecked(10..18))
};
```

---

## `cargo` output belongs in a different block than Rust code

A common mistake in technical books: mixing shell commands, cargo invocations, and program output in the same listing type. Give each its own source language:

```asciidoc
Build and run against the sample file:

[source,shell]
----
cargo run --release -- data/01302020.NASDAQ_ITCH50
----

Expected output (your numbers will vary by hardware):

[source,text]
----
Parsed 281,544,738 messages in 4.2s
Throughput: 67.0M messages/sec
Peak RSS: 42MB
----
```

The distinction matters in syntax highlighting and it signals to the reader what kind of content they're looking at before they read it.

---

## The truncated listing convention

You will often need to show a listing that evolves across chapters — the feed handler in ch01 is a stub; by ch05 it's hardened. Use `// ...` for elided code:

```rust
pub fn parse_message<'a>(buf: &'a [u8]) -> Result<Message<'a>, ParseError> {
    // ... error handling added in Chapter 5
    match buf[0] {
        b'A' => parse_add_order(&buf[1..]),
        // ...
    }
}
```

When you elide, note *why* — either "not yet implemented" or "unchanged from Listing X-Y." Never silently omit lines of important context.

---

## Typestate diagrams

Chapters 7 and 12 encode state transitions in the type system. These concepts are genuinely hard to follow in code alone. Place a plain ASCII diagram in the prose *before* the type definitions — when the reader hits the `PhantomData` and `impl<S: State>` blocks, they already have the mental model:

```
FeedHandler<Disconnected>
    │  .connect()
    ▼
FeedHandler<Connected>
    │  .subscribe()
    ▼
FeedHandler<Subscribed>
    │  .next_message()  ←──────┐
    ▼                           │
Message                  (loop)─┘
```

This belongs in the prose, not inside a code block. It is a diagram of the type system's intent, not a listing.

---

## Rust-specific source language tags

Quick reference for consistent AsciiDoc tagging throughout the book:

| Content | AsciiDoc tag |
|---|---|
| Rust source code | `[source,rust]` |
| Compiler errors and warnings | `[source,text]` |
| Shell / cargo commands | `[source,shell]` |
| Program output, benchmarks | `[source,text]` |
| TOML (Cargo.toml) | `[source,toml]` |
| Deliberately broken code | `[source,rust]` + WARNING admonition before it |