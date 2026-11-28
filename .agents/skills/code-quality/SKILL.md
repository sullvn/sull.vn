---
name: code-quality
description: Code quality standards to apply when writing or reviewing code.
---

## Naming

Name things for what they mean to the reader. Use the word someone in the field already knows, and name a thing for what it is in the problem, not for how it's stored, what type it has, or the story around it.

- Prefer the standard term, such as `Sample` over `Lesson`. Add a qualifier only when the plain term collides with one already in scope.
- Avoid storage words such as slot, entry, item and holder.
- Name a field for its role, not its type: `target: Turn`, not `turn: Turn`.
- Keep second person out of identifiers, and don't reuse a name from the standard library.

## Reading Order

Let the reader go from top to bottom. Deal with the exceptional cases first and return early, then let the main path read as a short list of steps.

Give a name to anything that needs a second look: a step with branches, a formula, or a non-trivial inline function.

## Data

Shape data for the questions asked of it.

- Keep each fact in one place, in the form it's most often read.
- Hand a function data that's already in the form it needs. If a function is mostly unpacking its input, change the input.
- Don't build a collection only to throw it away.
- Choose types that can hold only what's valid.
- Keep two types apart when they mean different things, even if they look alike.

## Reasons

Everything earns its place. For every constant, mechanic, dependency, allocation and exception to a rule, be able to state why it's there.

Where the reason is "it works better", measure it. If no reason can be found, remove the thing.

Prefer what the ecosystem already provides to a hand-written version, after checking what it costs.

Treat a lint exception as a prompt to find another formulation. Add one only when a tool leaves no alternative.

## Comments

Write comments that explain unusual logic or provide high-level summaries. Avoid low-value comments that merely restate what the code does, the types, or a library's own documentation.

Prefer comments that explain why over ones that explain what: the reason for a choice, the constraint it works around, or what would break otherwise. The code already shows what it does.

Keep the why evergreen. A comment must make sense to someone reading the code long after it was written, so never describe the change that introduced it or the discussion behind it. That belongs in the commit message or pull request.

A declaration's comment says what the thing is for and what it promises. Notes on how it works go inside the body.

Put the reason for a choice where a reader will ask about it:

- On a type, why its data has that shape.
- On a tuning constant, why it exists and what evidence chose its value.
- At the top of a crate or package, how its modules fit together.

Don't end a single-sentence comment with a period. With two or more sentences, end each one with a period.

When a comment takes more than one line, start and end it with an empty comment line. This applies to every language and comment style:

```ts
/**
 * Root font size
 *
 * Set in `rem` to respect user font preferences.
 *
 */
```

```rust
///
/// Parses a decimal byte.
///
/// # Errors
///
/// If `s` is not a number from 0 to 255.
///
```

A single sentence that wraps over several lines gets them too:

```rust
///
/// Counts down each dead snake's wait, respawning
/// those whose wait is over
///
```

A comment that fits on one line doesn't get the empty lines.

Put an empty line between the items of a bulleted list in a comment:

```rust
///
/// Each kind of data has its own shape:
///
/// - Orders are a map, because they're looked up by id.
///
/// - Events are a list, because they're read in order.
///
```

Never write a block comment on a single line, like `/* Example */` or `/** Example */`. Use a line comment (`//`) instead, or spread a doc comment over several lines:

```ts
/**
 * Example
 */
```

CSS is the exception, including `<style>` blocks in components, as block comments are its only kind of comment.

## Organization

Sort module-scope declarations with exports first, then non-exports. Within each group, sort topologically from the top down: a declaration appears after the declarations that use it, so a file reads from its entry points down to the details.

A file that fronts a directory is a table of contents. It holds only module declarations and re-exports.

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

## Rust

- Put unit tests in one `#[cfg(test)] mod tests` at the end of the file they test.
- Name a boolean function `is_` plus an adjective, like `is_empty`, or as a bare verb phrase, like `contains`.
- Keep a closure inline only when it's a single expression with no closure inside it and no `?` or `return`. Otherwise give it a name.
- Keep `lib.rs`, and any `foo.rs` beside a `foo/` directory, to `mod` lines, re-exports and the top comment.
- Use `Box<[T]>` for an owned buffer that never grows. Pass an iterator along instead of collecting it when the consumer only picks or counts.
- For a small fixed set of values, use an enum with a `match` table instead of integer arithmetic, so the compiler checks every case.
- Write a lint exception as `#[expect(…, reason = "…")]`.
