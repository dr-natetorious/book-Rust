# Rust for Engineers Who Ship Things

Project-driven Rust book repository with reproducible source code, confidence tests, and multi-format publishing.

## Repository Layout

1. .proposal: framing, chapter metadata, and editorial decisions.
2. chapters: AsciiDoc book files named number_title.adoc.
3. src: source-of-truth Rust code, organized by chapter.
4. test: confidence-focused tests and deterministic fixtures.
5. scripts: build and validation helpers.

## Core Rule

Do not duplicate code snippets inline in chapter prose if the same snippet exists in src. Use AsciiDoc tagged includes.

## Quick Start

1. Run Rust tests:

```bash
cargo test
```

2. Validate AsciiDoc includes:

```bash
bash scripts/validate-includes.sh
```

3. Build HTML/PDF/EPUB:

```bash
bash scripts/build-book.sh
```

## Docker Workflow

Build the image once:

```bash
docker build -t book-rust-toolchain .
```

Run chapter 1 tests in the container:

```bash
docker run --rm -v "$PWD:/workspace" book-rust-toolchain chapter 1
```

Run full tests:

```bash
docker run --rm -v "$PWD:/workspace" book-rust-toolchain test-all
```

Build the book outputs:

```bash
docker run --rm -v "$PWD:/workspace" book-rust-toolchain build-book
```

## Agent Configuration

1. VS Code/Copilot files live under .github.
2. Claude workflow files live under .claude.
3. Root and subfolder AGENTS.md files define repository rules and local extensions.
