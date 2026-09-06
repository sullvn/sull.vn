---
name: code-quality
description: Code quality standards to apply when writing or reviewing code.
---

## Comments

Write comments that explain unusual logic or provide high-level summaries. Avoid low-value comments that merely restate what the code does.

## Organization

Sort module-scope declarations with exports first, then non-exports. Within each group, sort topologically—a function should appear before the functions or constants that use it.

## Length

Avoid long functions and files. When they grow unwieldy, break them into smaller, focused units.

## Purity and Testability

Prefer pure functions over those with side effects. For functions that need side-effectful dependencies, pass them as arguments or via a context parameter (e.g., `env`). This enables easy mocking in tests.

## Test Location

Keep unit tests close to the code they test.
