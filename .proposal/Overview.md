# Book Proposal
## *Rust for Engineers Who Ship Things*
### Building a high-frequency trading platform from raw bytes to production crate

---

## The One-Line Pitch

A hands-on Rust book for senior engineers who already know how to program — built around a single project (an HFT data platform) that makes every concept feel necessary instead of academic.

---

## The Problem with Existing Rust Books

Every major Rust book teaches the language the same way: rules first, examples second. The reader spends 50 pages learning ownership theory before they write anything that does something real. By the time they hit lifetimes they've lost the thread entirely.

The Rust Programming Language (TRPL) is excellent documentation. It is not a learning experience. Rust in Action comes closer but still leans heavily on explanation over construction. Neither targets the senior engineer who already understands memory, concurrency, and systems design — and just needs to learn Rust's particular model for enforcing correctness at compile time.

This book takes the opposite approach. You build a thing. The concepts emerge from the build. The compiler errors you hit along the way are the syllabus.

---

## The Book

*Rust for Engineers Who Ship Things* is a project-driven book for engineers with production experience in systems languages (C, C++, Go) or high-level languages used in performance-sensitive contexts (Python, Java). It assumes the reader knows what a heap allocation costs, has debugged a race condition in production, and has strong opinions about API design. It does not assume any prior Rust experience.

The project is an HFT market data platform: a system that ingests a binary market feed, maintains a live order book, applies pre-trade risk checks, and publishes snapshots to downstream consumers. The platform is built chapter by chapter. By the final chapter the reader has a working, concurrent, tested system they can publish as a crate.

The finance domain is deliberately shallow. You don't need to know what a dark pool is. You need to know that this system processes a lot of data very fast, can't be wrong, and can't be slow. That's enough to make every Rust concept feel justified.

### What makes it different

**Learn by doing, not by reading about doing.** Every chapter has one exercise. Every exercise runs against real Nasdaq ITCH binary data, freely available from Nasdaq's public FTP server. The reader sees real output from the first page.

**Teach how, not about.** The prose is organized around concrete reader questions: how to make the next part work, why the naive implementation failed, what tradeoff now matters, and which evidence supports the next choice. Definitions follow contact with the problem.

**Senior engineer voice.** No hand-holding on concepts the reader already knows. No explaining what a thread is. The book meets the reader where they are and moves fast.

**Reader-led narrative.** The reader is the protagonist. Chapters track their attempts, compiler errors, measurements, and design decisions. The author does not stand at the front of the room explaining Rust from a distance.

**The compiler is the teacher.** Several exercises ask the reader to introduce a bug deliberately — a data race, a use-after-free equivalent, undefined behavior. The chapter is about understanding what happens and why the compiler either catches it or doesn't.

**One project, twelve chapters.** The platform is the spine. Every chapter adds a real component. There is no throwaway tutorial code. Everything the reader writes in chapter 2 is still running in chapter 12.

**Honest about scope.** The platform is not production HFT infrastructure. It couldn't trade in a colo. But by the epilogue the reader understands exactly what it would take to get there — and the PyO3 bridge to Python data science is the more realistic next step for most of them.

---

## Target Reader

**Primary:** Senior software engineers (5+ years) coming from C++, Go, Python, or Java who want to add Rust to their toolkit for performance-sensitive work. They do not need prior Rust depth; they need a project that teaches Rust by making it necessary.

**Secondary:** Engineers in finance, systems infrastructure, or data engineering who want a credible applied context for learning Rust. They recognize the HFT domain and respect its constraints without needing it explained.

**Not for:** Complete programming beginners. Engineers looking for Rust compiler internals, procedural macros, or custom allocators (that's a different book). Anyone who needs the finance domain explained from first principles.

### What the reader arrives with (L200)

- At least one systems or high-level language at production depth
- Understanding of heap vs stack, what a pointer is, why thread safety matters
- Willingness to read unfamiliar Rust syntax in context while building the platform
- Enough market domain awareness to not need a glossary

### What the reader leaves with (L300)

- Ownership, borrowing, and lifetimes — internalized, not memorized
- Idiomatic error handling, traits, and generics with real tradeoff intuition
- Concurrent and async Rust that compiles without data races — by design, not luck
- Unsafe Rust used correctly, with Miri as a safety net
- A published crate with a versioned public API
- A working PyO3 bridge to Python for data science downstream

---

## The Project: A Market Data Platform

The platform is built incrementally across 12 chapters. Each chapter adds one component. The reader always has working code at the end of every chapter.

```
Feed handler → Order book → Risk engine → Concurrent pipeline → Async fan-out → Published crate
```

**Data source:** Free Nasdaq ITCH 5.0 binary sample files from `emi.nasdaq.com/ITCH/`. No account required. Files range from 3.3GB to 5.2GB compressed. The Jan 30 2020 file (WHO declares COVID-19 a PHEI) is the recommended sample — elevated volatility, high message volume, interesting microstructure. Readers run every exercise against real historical market data.

**Why this domain works:**

- Binary protocol parsing makes ownership and zero-copy feel necessary, not academic
- High throughput (500k+ messages/second) makes allocation discipline feel urgent
- ITCH message types map naturally to Rust enums — Add Order, Execute, Cancel, Replace
- Natural concurrency pressure: parse on one thread, maintain book state on another
- Free public data means every reader runs identical exercises against identical input
- The epilogue extends naturally into Python data science via PyO3 — which is where most readers actually want to go

---

## The Learning Model

### How to teach — the Rosetta Stone principle

The book does not explain concepts and then show examples. It creates the conditions for the reader to discover concepts by building things that don't work yet.

The chapter-level question is almost always procedural: how do you make this work, why did that fail, what changed when you measured it, or what constraint just became real. "What is X?" appears when needed, but it is not the engine of the book.

Each chapter follows the same structure:

1. **The problem** — a real component of the platform that needs to be built
2. **The friction** — what breaks when you try to build it naively
3. **The concept** — the Rust feature that resolves the friction
4. **The exercise** — build the component correctly, against real data
5. **The callback** — the next chapter opens by referencing what was just built

There are no chapter debriefs. The next chapter is the debrief.

Progressive discovery is non-negotiable. The reader sees code, output, errors, or measurements before they get the more abstract explanation. The concept lands after the need for it exists.

### Domain context rule

One paragraph of finance context per chapter, maximum. Placed exactly when needed. Never front-loaded. The reader is never asked to understand market microstructure to understand the Rust concept.

Example of how it lands:

> *"Orders get cancelled far more than they execute. On a busy day, cancel messages outnumber fills 10 to 1. That's why the hot path is the cancel handler, not the trade handler — and why every allocation on that path is money left on the table."*

One sentence. Pays off immediately. Never returns.

### The throughline

Three acts underneath the five parts:

**Act one (ch. 1–4) — Unlearning.** The reader's existing instincts are wrong here. The compiler is right. The emotional beat is frustration → recognition.

**Act two (ch. 5–10) — Building with confidence.** Not fighting anymore. Making deliberate choices with evidence. The emotional beat is competence → fluency.

**Act three (ch. 11–12) — Trust.** Trusting the compiler enough to wrap unsafe C code and ship an API other people depend on. The emotional beat is fluency → mastery.

### Prose shape

The prose should sound authored, not templated. Sentence length varies. Paragraph shape varies. Straightforward setup moves quickly; hard ideas get more space. A chapter where every section has the same rhythm or the same amount of explanation is structurally wrong even if every fact is correct.

---

## Table of Contents

### Preamble — Before you write a line of Rust
*pp. 1–18 · Motivation · toolchain · compiler errors · first Cargo project*

- The problem Rust was built to solve — p. 2
- Who this book is for — and what it assumes — p. 4
- The platform you're going to build — p. 6
- Toolchain setup: rustup, clippy, rustfmt, rust-analyzer — p. 9
- How to read a compiler error — p. 12
- Your first Cargo project — zero to compiled — p. 14
- Downloading the sample data — p. 17

---

### Part One — Getting Data In
*pp. 19–58*

#### Chapter 1 — Parse a binary market feed from raw bytes
*Ownership · move semantics · the drop model*

- How a market feed works — the one paragraph you need — p. 20
- Reading binary: big-endian, fixed-width, no ceremony — p. 22
- Ownership and the drop model — values have exactly one home — p. 25
- Move semantics: why the compiler stops you before you make a mistake — p. 29
- **Exercise:** print every Add Order from the sample file — p. 34

#### Chapter 2 — Handle all message types without allocating on the hot path
*Borrowing · references · zero-copy slicing*

- Why allocations matter at 500k messages per second — p. 39
- Borrowing and zero-copy slicing — views into existing memory — p. 41
- The aliasing XOR mutability rule — p. 46
- **Exercise:** complete the feed handler, verify zero allocations — p. 52

---

### Part Two — Modeling State
*pp. 59–96*

#### Chapter 3 — Build an order book that cannot hold a contradictory state
*Enums as tagged unions · exhaustive match · type-driven design*

- What an order book is — the two-minute version — p. 60
- Enums as tagged unions, not glorified constants — p. 62
- Exhaustive match — the compiler as a logic checker — p. 66
- Making illegal state transitions unrepresentable — p. 70
- **Exercise:** replay a full day, zero panics — p. 74

#### Chapter 4 — Stream a full trading day without running out of memory
*Iterators · lazy evaluation · allocation discipline*

- Why a 5GB file doesn't need 5GB of RAM — p. 79
- Iterators: lazy by default, zero-cost by design — p. 81
- Implementing Iterator on your own types — p. 85
- Verifying zero-overhead abstraction with cargo-asm — p. 89
- **Exercise:** flat memory profile across the full sample file — p. 92

---

### Part Three — Correctness Under Pressure
*pp. 97–152*

#### Chapter 5 — Handle every failure without crashing the platform
*Result · the ? operator · error type design · testable errors*

- What corrupt feed data actually looks like — p. 98
- Result and the ? operator as a design philosophy — p. 100
- Designing error types callers can act on — p. 104
- When to panic, when to propagate — and how to test both — p. 108
- **Exercise:** harden the feed handler against deliberate corruption — p. 113

*Testing is introduced here as a property of good error design — testable error types look structurally different from untestable ones. No standalone testing chapter needed.*

#### Chapter 6 — Add a pre-trade risk check the type system enforces
*Traits · generics · monomorphization vs dynamic dispatch*

- What a risk check is — position limits, notional caps, rate limits — p. 118
- Traits: behavior without inheritance — p. 120
- Generics and monomorphization — what the compiler does for you — p. 124
- Dynamic dispatch — when you pay for flexibility — p. 128
- **Exercise:** two implementations, one flamegraph, one decision — p. 132

#### Chapter 7 — Share live risk state across the platform without copying it
*Lifetimes · smart pointers · interior mutability*

- Why three components need the same data at the same time — p. 137
- Lifetimes: naming what the compiler already knows — p. 139
- Box, Rc, Arc — the ownership decision tree — p. 143
- Interior mutability: RefCell and when the borrow check moves to runtime — p. 147
- **Exercise:** shared risk state, working code at every fork — p. 150

---

### Part Four — Speed
*pp. 153–206*

#### Chapter 8 — Find and fix the allocation killing your throughput
*Profiling · flamegraphs · cargo-asm · evidence-based optimization*

- Establishing your baseline before you touch anything — p. 154
- cargo-flamegraph: finding the hot path — p. 157
- cargo-asm: reading what the compiler actually produced — p. 161
- One change, one measurement — the discipline of evidence — p. 165
- **Exercise:** hit 1M+ messages per second on commodity hardware — p. 168
- Threads vs async — which model fits which problem — p. 171

*The threads vs async comparison lives here as a transition beat — bridges into ch. 9 and 10 without either chapter restating it.*

#### Chapter 9 — Process the feed and update the book at the same time
*Threads · Send + Sync · channels · no data races*

- Send and Sync: the type system as a race detector — p. 175
- Channels: ownership transfer as a concurrency primitive — p. 179
- Introducing a data race deliberately — and watching the compiler refuse — p. 183
- **Exercise:** parser thread, book thread, measured throughput gain — p. 187

#### Chapter 10 — Handle a live feed without blocking on slow consumers
*Async/await · Tokio · backpressure · tokio-console*

- Futures and the state machine the compiler generates — p. 192
- Tokio as infrastructure, not magic — p. 196
- The mutex-across-await deadlock — live diagnosis with tokio-console — p. 199
- **Exercise:** async fan-out to risk, book, and logger consumers — p. 203

---

### Part Five — Ship It
*pp. 207–251*

#### Chapter 11 — Decompress the sample data without leaving Rust
*Unsafe · FFI · bindgen · Miri · safety contracts*

- The problem: you've been unzipping files manually since page one — p. 208
- The five unsafe superpowers — what you're actually permitted to do — p. 210
- Wrapping libz: bindgen and ownership at the FFI boundary — p. 214
- Safety contracts and Miri — introduce UB, catch it, ship the fix — p. 220
- **Exercise:** one binary, downloads and decompresses the sample file itself — p. 225

*Motivation is real — the reader has needed this since the preamble. The chapter teaches unsafe/FFI through a problem the reader already has, not a contrived finance example.*

---

*The platform works. Now make it something another team can depend on.*

---

#### Chapter 12 — Package the platform so another team can depend on it
*API design · typestate · sealed traits · semver discipline*

- What makes an API survive its second version — p. 230
- Typestate: making misuse a compile error — p. 233
- Sealed traits: controlling what callers can implement — p. 238
- Semver discipline — breaking changes as a design decision, not an accident — p. 242
- **Exercise:** publish the feed handler, make a breaking change, fix it right — p. 246

---

### Epilogue — The engine runs. Now put a cockpit on it.
*pp. 251–257*

- Calling your Rust feed handler from Python with PyO3 — a working example in 50 lines — p. 252
- Feeding order book snapshots into pandas — from raw bytes to a DataFrame — p. 254
- Where the quant stack goes from here — signals, prediction, backtesting — p. 255
- The FPGA horizon — what it would take, and who's already there — p. 256
- Serious Rust source you're ready to read — bytes, tokio, crossbeam — p. 257

*Further reading and resources live at the companion GitHub repo. Not in the book.*

---

## Page Budget

| Section | Chapters | Pages |
|---|---|---|
| Preamble | — | 18 |
| Part I: Getting data in | 1–2 | 40 |
| Part II: Modeling state | 3–4 | 38 |
| Part III: Correctness under pressure | 5–7 | 56 |
| Part IV: Speed | 8–10 | 54 |
| Part V: Ship it | 11–12 | 45 |
| Epilogue | — | 7 |
| **Total (prose)** | | **~258** |

**Print estimate with art and code listings:** 275–285 pages

Target Kindle price: $39.99  
Target print price: $49.99

---

## Companion Materials

**GitHub repo** (public, linked from book):
- All exercise starter code and solutions
- The complete platform as a reference implementation
- Further reading and resource links
- Errata

**Sample data:**
- Nasdaq ITCH 5.0 files at `emi.nasdaq.com/ITCH/Nasdaq%20ITCH/`
- No account required. Recommended: Jan 30 2020 (5.2GB compressed)
- Dec 30 2019 (3.3GB) for readers on slower connections

---

## Competitive Landscape

| Book | Level | Approach | Gap |
|---|---|---|---|
| The Rust Programming Language | L100–L200 | Reference documentation | Not a learning experience |
| Rust in Action | L100–L200 | Concept-first with examples | Too much explanation, not enough building |
| Programming Rust (O'Reilly) | L200–L400 | Comprehensive reference | Dense, not project-driven |
| Zero to Production in Rust | L200–L300 | Web services | Different domain, web-focused |
| *This book* | L200→L300 | Project-driven, systems domain | Senior engineers, HFT platform spine |

The closest competitor is Zero to Production in Rust — project-driven, targets working engineers. The gap is domain and level: that book builds a web service and spends significant time on concepts this book's reader already knows. This book targets systems engineers and moves faster.

---

## What This Book Is Not

- **Not a reference.** The Rust Programming Language exists. This book sends you there when you need it.
- **Not a finance book.** The domain is set dressing. No options theory, no market microstructure, no trading strategy.
- **Not L500.** Procedural macros, custom allocators, compiler plugins, LLVM internals — out of scope. That's a different book.
- **Not a colo trading system.** The platform is an educational project. The epilogue is honest about the gap between what you built and what a real HFT firm runs.

---

## The Epilogue Promise

The platform you build in this book processes market data on commodity hardware. A colocated FPGA running the same logic at Carteret, NJ clocks in under 25 nanoseconds per message. The Columbia paper "Building the Book: A Full-Hardware Nasdaq ITCH Ticker Plant" shows exactly what that looks like — including a hardware AVL tree for O(log n) order lookup and a compiler that auto-generates Verilog from message descriptions.

That's the door. This book doesn't walk through it. But by the epilogue you know exactly where it is.

The more immediate next step for most readers is PyO3: expose your Rust feed handler as a Python module, import it in a Jupyter notebook, feed order book snapshots into pandas or a simple prediction model. The engine you built is already fast enough for real data science work. The epilogue shows you how to connect the two in 50 lines of code.

---

## Editorial Standing Instructions

These decisions were arrived at deliberately and should not be revisited without strong reason:

1. **Every chapter opens with a callback to the previous exercise.** No chapter starts cold.
2. **Domain context is one paragraph per chapter, maximum.** Never front-loaded.
3. **Every exercise runs against real Nasdaq sample data.** No synthetic toy examples.
4. **No chapter debriefs.** The next chapter's opening is the debrief.
5. **Testing is introduced in ch. 5** as a property of good error type design. No standalone testing chapter.
6. **Threads vs async comparison lives at the end of ch. 8**, not the start of ch. 9 or 10.
7. **Further reading lives in the GitHub repo.** Not in the book.
8. **The FFI chapter (ch. 11) uses libz, not a finance library.** Motivation is real — the reader has needed it since the preamble.
9. **"How to use this book" is not a section.** The book is self-evident.
10. **The epilogue leads with PyO3/Python, not FPGA.** FPGA is one paragraph. Python is the real next step for 95% of readers.
11. **The book teaches how to do the work, not abstractly about Rust concepts.**
12. **The reader is the protagonist.** Chapters follow their actions, failures, discoveries, and progress.
13. **Progressive discovery is mandatory.** Errors, measurements, outputs, and broken attempts come before extended explanation.
14. **Prose must vary enough to sound human.** Repetitive cadence or uniform paragraph structure is a draft failure, not a stylistic quirk.
15. **Section length follows complexity and payoff.** Important ideas get more room; low-complexity setup gets less.

---

## Author Background

- Principal Software Engineer, JPMC Office of the CTO Architecture
- Previously: Principal Solutions Architect at AWS, Senior SRE at Citadel (options team), Senior SDE at Microsoft
- Published author: *Engineering Resilient Systems on AWS* (O'Reilly)
- Domain experience: distributed systems, low-latency infrastructure, options trading systems
- Active Rust learner — building an ITCH feed processor as the direct motivation for this book

---

*This proposal was developed through an iterative editorial process. The structure, level, domain framing, chapter sequence, page budget, and editorial standing instructions are all the result of explicit decisions, not defaults. The book described here is the book that should be written.*