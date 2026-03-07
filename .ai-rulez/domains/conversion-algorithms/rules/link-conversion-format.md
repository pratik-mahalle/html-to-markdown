---
name: Link Conversion Format
priority: high
---
Convert links to Markdown [text](url) format

- Extract href and convert to [text](href)
- Preserve link text with inline formatting
- Optional: Include title attribute as "title"
- Sanitize URLs to prevent XSS (use sanitize_url())
- Handle fragment URLs (#anchor)
- Fallback to URL if no text
- Support reference-style links if configured
- Test with various URL schemes (http, https, mailto, #)
