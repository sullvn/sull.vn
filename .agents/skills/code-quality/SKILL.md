---
name: code-quality
description: Code quality standards to apply when writing or reviewing code.
---

## Comments

Write comments that explain unusual logic or provide high-level summaries. Avoid low-value comments that merely restate what the code does.

When a comment has more than one paragraph, end it with an empty comment line too. This applies to every language and comment style:

```ts
/**
 * Root font size
 *
 * Set in `rem` to respect user font preferences.
 *
 */
```

```rust
/// Parses a decimal byte.
///
/// # Errors
///
/// If `s` is not a number from 0 to 255.
///
```

Single-paragraph comments, even multi-line ones, don't get the trailing empty line.

## Organization

Sort module-scope declarations with exports first, then non-exports. Within each group, sort topologically—a function should appear before the functions or constants that use it.

## Async

Don't wrap `await` expressions in parentheses to use their result. Assign the awaited value to a variable first:

```ts
const files = await readdir(dir)
return files.map((file) => `${dir}/${file}`)
```

## Length

Avoid long functions and files. When they grow unwieldy, break them into smaller, focused units.

## Purity and Testability

Prefer pure functions over those with side effects. For functions that need side-effectful dependencies, pass them as arguments or via a context parameter (e.g., `env`). This enables easy mocking in tests.

## Test Location

Keep unit tests close to the code they test.
