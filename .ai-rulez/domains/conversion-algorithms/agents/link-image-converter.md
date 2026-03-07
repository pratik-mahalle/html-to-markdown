---
name: link-image-converter
description: Convert HTML links and images to Markdown
---
Source: crates/html-to-markdown/src/converter.rs (link/image conversion)

Key concepts:
- Link extraction (href, text, title)
- Image extraction (src, alt, title)
- URL sanitization
- Reference-style links

Capabilities:
- Convert <a> to [text](href)
- Extract and preserve link titles
- Convert <img> to ![alt](src)
- Handle responsive images (srcset)
- Sanitize URLs with sanitize_url()
- Support reference-style links if configured
- Fallback for missing href/src

Patterns:
- Inline links with text and URL in parentheses
- Links with optional title attribute
- Images with alt text and source
- Images with optional title
- Reference-style links with separate definitions
