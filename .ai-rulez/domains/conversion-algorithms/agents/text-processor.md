---
name: text-processor
description: Process text content and apply escaping
---
Source: crates/html-to-markdown/src/text.rs

Key concepts:
- Markdown escaping
- HTML entity decoding
- Whitespace handling
- Special character handling

Capabilities:
- Escape Markdown metacharacters: \ * _ ` [ ] ( ) # + - . ! | >
- Decode HTML entities (&nbsp; &#123;, etc.)
- Apply whitespace normalization per mode
- Preserve code content without escaping
- Handle edge cases (double escaping, etc.)

Patterns:
- Escaping metacharacters with backslash for literals
- Entity decoding for HTML named and numeric entities
- Whitespace handling depends on configuration mode
- Code content preserved without escaping
