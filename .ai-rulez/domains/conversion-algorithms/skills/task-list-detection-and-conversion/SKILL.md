---
description: "Task List Detection And Conversion"
name: task-list-detection-and-conversion
---
Detect and convert checkbox task lists

Key source files:
- crates/html-to-markdown/src/converter.rs

Master concepts:
- Checkbox detection
- Checked state extraction
- Task list format ([ ] vs [x])
- Item content processing

Step by step:
1. For each li element, check for input checkbox
2. If found
   a. Extract checked attribute
   b. Format as - [ ] text for unchecked
   c. Format as - [x] text for checked
3. Process list item content normally
4. Handle mixed task and regular lists
