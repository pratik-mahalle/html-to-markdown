---
name: Text Content Preservation
priority: high
---
Extract text content preserving whitespace exactly

- Implement extract_text() for full subtree content
- Preserve all whitespace in text nodes (spaces, tabs, newlines)
- Don't apply HTML5 whitespace normalization
- Decode HTML entities (&nbsp;, &lt;, &#123;, etc.)
- Preserve CDATA sections as literal text
- Handle special characters correctly
