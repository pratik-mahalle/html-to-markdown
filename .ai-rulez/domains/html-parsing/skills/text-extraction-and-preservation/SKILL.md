---
description: "Text Extraction And Preservation"
name: text-extraction-and-preservation
---

Extract text content preserving original whitespace

Key source files:

- crates/html-to-markdown/src/text.rs

Master concepts:

- Whitespace preservation vs normalization
- HTML entity decoding
- Control character handling
- Text node collection

Step by step:

1. Traverse element recursively
2. Collect all text nodes
3. Decode HTML entities (e.g., &nbsp; becomes space)
4. Apply whitespace mode
   - Preserve: keep as-is
   - Minimal: trim and normalize internal
   - Collapse: HTML5-style collapse
5. Handle special cases
   - CDATA sections
   - Script/style content (skip)
6. Return accumulated text
