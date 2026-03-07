---
name: emphasis-and-formatting-conversion
---
Convert text emphasis tags to Markdown

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Bold/strong formatting
- Italic/emphasis formatting
- Strikethrough
- Nested emphasis
- Escaping rules

Step by step:
1. For strong tags wrap with double asterisks
2. For em tags wrap with single asterisk
3. For del/s tags wrap with double tildes
4. For nested emphasis (bold and italic together)
   a. Detect combination
   b. Use triple asterisks
5. Avoid double-escaping with too many marks
6. Preserve content without interpretation
