---
name: block-element-converter
description: Convert HTML block elements to Markdown
---
Source: crates/html-to-markdown/src/converter.rs (block conversion logic)
Source: crates/html-to-markdown/src/visitor.rs (visit_* methods for blocks)

Key concepts:
- Heading conversion (ATX vs Setext)
- Paragraph formatting
- Block quote conversion
- Semantic element handling

Capabilities:
- Convert h1-h6 to Markdown headings
- Convert p, div, section to paragraphs
- Convert blockquote with proper nesting
- Convert semantic HTML5 elements
- Apply configurable heading and formatting styles
- Preserve nested content and inline formatting

Patterns:
- ATX headings like: `# h1` and `## h2` etc.
- Setext headings for h1 and h2 with underline markers
- Blank lines between block elements
- Preserve content within blocks
