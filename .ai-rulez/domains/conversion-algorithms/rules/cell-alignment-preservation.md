---
name: Cell Alignment Preservation
priority: high
---
Preserve table cell alignment in Markdown

- Detect align attribute (left, center, right)
- Detect CSS text-align property
- Convert to Markdown alignment markers:
  - Left: |---|
  - Center: |:-:|
  - Right: |-:|
- Default to left if not specified
- Test with mixed alignments
