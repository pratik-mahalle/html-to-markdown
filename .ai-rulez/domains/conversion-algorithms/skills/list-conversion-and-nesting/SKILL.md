---
name: list-conversion-and-nesting
---
Convert HTML lists to Markdown lists with proper nesting

Key source files:
- crates/html-to-markdown/src/converter.rs (convert_list)

Master concepts:
- Unordered vs ordered lists
- List item processing
- Nesting and indentation
- Tight vs loose detection

Step by step:
1. Identify list type (ul or ol)
2. Process each list item
   a. Extract item content
   b. Process nested lists recursively
3. Determine indentation level
4. Choose bullet/number style
   - ul uses -, *, or + per option
   - ol uses 1., 2., etc.
5. Detect tight/loose list formatting
   - Tight means no blank lines between items
   - Loose means blank lines between items
6. Format output with proper indentation
