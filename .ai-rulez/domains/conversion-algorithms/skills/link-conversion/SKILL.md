---
description: "Link Conversion"
name: link-conversion
---

Convert HTML links to Markdown format

Key source files:

- crates/html-to-markdown/src/converter.rs (convert_link)

Master concepts:

- href extraction
- Link text processing
- Title attribute handling
- URL sanitization

Step by step:

1. Extract href attribute
2. Sanitize URL with sanitize_url()
3. Extract link text
4. Check for title attribute
5. Format as [text] with href in parentheses
6. Handle edge cases
   - No href means treat as plain text
   - No text means use href as text
   - Fragment URLs preserve anchor
