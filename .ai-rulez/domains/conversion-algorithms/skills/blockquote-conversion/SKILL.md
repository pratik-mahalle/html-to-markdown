---
description: "Blockquote Conversion"
name: blockquote-conversion
---
Convert blockquote elements to Markdown

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Block quote prefix (>)
- Nesting support
- Content preservation
- Line-by-line formatting

Step by step:
1. Extract blockquote content
2. Split by lines
3. For each line, prefix with greater-than and space
4. For nested blockquotes add multiple greater-than markers
5. Preserve blank lines within blockquote
6. Handle multi-line content with multiple paragraphs
