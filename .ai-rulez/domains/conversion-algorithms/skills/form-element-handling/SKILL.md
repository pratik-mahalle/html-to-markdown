---
description: "Form Element Handling"
name: form-element-handling
---
Convert form elements to Markdown equivalents

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Input type detection
- Label association
- Fallback formats

Step by step:
1. Identify input type
   - text becomes code block or input notation
   - checkbox becomes task list
   - radio becomes list item with marker
   - hidden is skipped entirely
2. Extract label from label element or associated text
3. Convert based on type
   - Text inputs become code block with label
   - Checkbox inputs become task list
   - Select becomes list of options with selection marked
4. Preserve field structure
