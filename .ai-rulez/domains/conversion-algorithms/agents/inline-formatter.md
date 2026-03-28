---
name: inline-formatter
description: Convert HTML inline formatting to Markdown
---

Source: crates/html-to-markdown/src/converter.rs (inline conversion)

Key concepts:

- Bold/italic/emphasis conversion
- Code and monospace handling
- Strikethrough conversion
- Emphasis nesting

Capabilities:

- Convert strong/b to **text**
- Convert em/i to *text*
- Convert del/s to ~~text~~
- Convert code/kbd/samp to `text`
- Handle nested emphasis (bold+italic)
- Avoid double-escaping
- Preserve content within formatting

Patterns:

- Bold text wrapped in double asterisks
- Italic text wrapped in single asterisks
- Code text wrapped in backticks
- Strikethrough text wrapped in tildes
- Combined bold and italic with triple asterisks
