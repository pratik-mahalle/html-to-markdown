---
name: text-processor
description: Extract and process text content with whitespace handling
---

Source: crates/html-to-markdown/src/text.rs

Key concepts:

- Text node extraction
- Whitespace preservation vs normalization
- HTML entity decoding
- Control character handling

Capabilities:

- Extract text preserving original whitespace
- Apply different whitespace modes (Preserve, Minimal, Collapse)
- Decode HTML entities correctly
- Handle special characters and encodings
- Normalize whitespace per configuration

Patterns:

- Preserve mode keeps all whitespace as-is
- Minimal mode trims and normalizes internal spacing
- Collapse mode applies HTML5-style whitespace rules
- Entity decoding handles named and numeric entities
