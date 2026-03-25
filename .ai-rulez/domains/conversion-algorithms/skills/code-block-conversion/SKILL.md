---
description: "Code Block Conversion"
name: code-block-conversion
---
Convert preformatted and code blocks

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Triple backtick format
- Language annotation
- Whitespace preservation
- Indentation style

Step by step:
1. Extract code content
2. Check for language hint (class, data-lang)
3. Determine style of backticks or indentation
4. If using backticks
   a. Use fence marker
   b. Add language tag if available
   c. Content on next lines
5. Preserve all whitespace exactly
6. Escape backticks in content if needed
