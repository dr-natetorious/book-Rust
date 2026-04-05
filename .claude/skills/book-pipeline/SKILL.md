---
name: book-pipeline
description: "Use when writing or revising chapter prose with source tags, tests, and manifest updates"
---

# Claude Book Pipeline Skill

Use this workflow when drafting or revising chapters:

1. Update chapters/NN_title.adoc with callback opening.
2. Pull code examples from src/chNN using tag includes.
3. Add confidence tests for changed behavior.
4. Update .proposal/manifest.yaml status.
5. Verify with cargo test and scripts/validate-includes.sh.
