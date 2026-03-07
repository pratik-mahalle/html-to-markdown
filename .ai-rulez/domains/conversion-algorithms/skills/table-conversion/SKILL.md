---
name: table-conversion
---
Convert HTML tables to Markdown GFM pipe tables

Key source files:
- crates/html-to-markdown/src/converter.rs (convert_table)

Master concepts:
- Table structure (thead, tbody, tfoot)
- Cell content and alignment
- GFM pipe format
- Header detection

Step by step:
1. Extract table structure
   - Headers from thead or th elements
   - Data from tbody or td elements
2. Collect column alignments
   - Extract from align attribute or CSS
   - Map to left, center, or right
3. Generate header row with pipes and content
4. Generate separator row with dashes and colons
   - dashes only for left alignment
   - dashes with colons for center
   - dashes with right colon for right
5. Generate data rows with proper escaping
6. Handle edge cases
   - Missing cells are filled with empty space
   - Pipe in content is escaped with backslash
