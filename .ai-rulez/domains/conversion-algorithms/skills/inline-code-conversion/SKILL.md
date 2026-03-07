---
name: inline-code-conversion
---
Convert code elements to Markdown

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Backtick wrapping
- Content preservation
- Nested code handling
- Escape rules

Step by step:
1. Extract code content
2. Determine backtick count needed
   - No backticks in content means use single backtick
   - Backticks present means use double or more
3. Wrap code with appropriate backtick count
4. Preserve content exactly without interpretation
5. Handle edge cases
   - Code with backticks
   - Code with special characters
   - Code with entity references
