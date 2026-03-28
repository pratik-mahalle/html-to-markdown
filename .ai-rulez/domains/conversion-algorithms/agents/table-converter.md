---
name: table-converter
description: Convert HTML tables to Markdown tables
---

Source: crates/html-to-markdown/src/converter.rs (table conversion)

Key concepts:

- Table structure (thead, tbody, tfoot)
- Cell content and alignment
- Rowspan/colspan handling
- Table header detection

Capabilities:

- Convert table to GFM pipe format
- Extract headers from thead or th elements
- Map data to rows and columns
- Preserve cell alignment (left, center, right)
- Handle missing cells and uneven rows
- Deal with simple colspan/rowspan
- Escape pipe characters in cells
- Fallback to list format for complex tables

Patterns:

- GFM pipe tables with headers and separators
- Alignment indicated by dashes and colons
- Cell content preserves inline formatting
- Captions extracted as text before table
