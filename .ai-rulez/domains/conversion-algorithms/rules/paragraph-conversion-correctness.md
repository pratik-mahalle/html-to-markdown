---
name: Paragraph Conversion Correctness
priority: high
---

Convert paragraphs to Markdown paragraphs

- Convert <p> to text followed by blank line
- Preserve inline formatting within paragraphs
- Handle nested inline elements (bold, italic, links)
- Remove extra whitespace but preserve content
- Support multi-line paragraphs (from <br> within <p>)
- Handle paragraphs with only whitespace
