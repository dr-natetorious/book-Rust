---
agent: ask
description: "Bootstrap a new chapter file, source tags, and baseline tests"
---

Create or update a chapter by following this checklist:

1. Add or update chapters/NN_title.adoc with callback opening and short domain context.
2. Add source under src/chNN with tag::/end:: boundaries for each included snippet.
3. Add confidence tests under test/chNN and wire a top-level integration test if needed.
4. Update .proposal/manifest.yaml chapter status and source/test paths.
5. Ensure cargo test passes for changed chapter scope.
