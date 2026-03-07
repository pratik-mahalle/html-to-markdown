---
name: Bold & Italic Conversion
priority: high
---
Convert emphasis tags to Markdown

- Bold: <strong>, <b> → **text**
- Italic: <em>, <i> → *text*
- Support nested emphasis (e.g., bold+italic → ***text***)
- Avoid double-escaping (no "****text****")
- Preserve emphasis across element boundaries
- Test with various nesting patterns
