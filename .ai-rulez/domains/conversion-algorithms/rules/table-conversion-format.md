---
name: Table Conversion Format
priority: high
---
Convert tables to Markdown pipe format

- Use GFM pipe table format (|---|---|)
- Extract table headers from <thead> or <th> elements
- Map table data to rows and columns
- Preserve cell alignment (left, center, right)
- Escape pipe characters in cell content
- Handle missing cells (uneven rows)
- Support simple colspan/rowspan (split or merge)
- Provide fallback to list format for complex tables
