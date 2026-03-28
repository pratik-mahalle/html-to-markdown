---
description: "Paragraph Conversion"
name: paragraph-conversion
---

Convert paragraph tags to Markdown

Key source files:

- crates/html-to-markdown/src/converter.rs (convert_p)

Master concepts:

- Content extraction
- Inline formatting preservation
- Blank line separation
- Whitespace handling

Step by step:

1. Extract paragraph content
2. Process inline formatting
   - Bold: convert strong tags to **text**
   - Italic: convert em tags to *text*
   - Links: convert a tags to [text](href)
3. Handle line breaks within paragraph
4. Trim excessive whitespace
5. Add blank line after paragraph
