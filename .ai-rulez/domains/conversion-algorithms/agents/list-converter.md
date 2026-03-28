---
name: list-converter
description: Convert HTML lists to Markdown lists
---

Source: crates/html-to-markdown/src/converter.rs (list conversion)

Key concepts:

- Unordered list conversion
- Ordered list conversion
- List nesting and indentation
- List item processing
- Task list detection

Capabilities:

- Convert ul/ol to Markdown lists
- Handle nested lists (multiple levels)
- Configure list bullet style (-, *, +)
- Configure list indentation (spaces or tab)
- Detect and convert task lists ([ ])
- Preserve list item content with formatting
- Handle tight vs loose list detection

Patterns:

- Unordered items use -, *, or + prefix
- Ordered items use 1., 2., etc.
- Nested items have proper indentation at each level
- Task items marked with [x] or [ ]
