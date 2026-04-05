---
name: book-pipeline
description: "Use when creating or updating chapter prose, source tags, tests, manifest status, and docker-validated workflows"
---

# Book Pipeline Skill

## Workflow

1. Read chapter brief from .proposal/chapters.
2. Update chapter AsciiDoc file with callback and exercise framing.
3. Implement or update source under src/chNN with tag markers.
4. Add or update confidence tests in test/chNN and integration entry points.
5. Update .proposal/manifest.yaml status and paths.
6. Run chapter tests and include validation.

## Constraints

1. No duplicated source code in prose if tagged include exists.
2. Keep code deterministic and fixture-driven.
3. Preserve compatibility with docker-entrypoint chapter commands.
