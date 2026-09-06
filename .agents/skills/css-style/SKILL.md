---
name: css-style
description: CSS coding standards to apply when writing or reviewing styles.
---

## Theming

Consider both light and dark mode when defining colors and styles.

## Values

Avoid hardcoding values. Reference defined constants or CSS variables instead.

When choosing values, prefer in this order:
1. Reuse or reference existing values
2. Use values that fit into the design system
3. Hardcode a new value only as a last resort

## Layout

Define layouts via flexbox or grid on the container. Use `gap` and `padding` for spacing. Avoid margins on layout items.

## Units

Use text-relative units (`em`, `ex`, `ch`, etc.) when the value should scale with the surrounding text.

Use root units (`rem`, `rch`, etc.) when the value should remain consistent regardless of surrounding content.
