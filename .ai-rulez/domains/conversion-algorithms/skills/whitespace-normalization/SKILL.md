---
description: "Whitespace Normalization"
name: whitespace-normalization
---
Normalize whitespace per configuration

Key source files:
- crates/html-to-markdown/src/options.rs (WhitespaceMode)

Master concepts:
- Preserve mode (identity)
- Minimal mode (trim, normalize internal)
- Collapse mode (HTML5 rules)

Step by step:
1. Get WhitespaceMode from options
2. Apply based on mode
   - Preserve means no change
   - Minimal means trim and normalize
   - Collapse means apply HTML5 rules
3. HTML5 collapse logic
   a. Replace newline and whitespace with space
   b. Collapse multiple spaces to one
   c. Remove leading and trailing as context-dependent
4. Preserve line breaks from br tags
