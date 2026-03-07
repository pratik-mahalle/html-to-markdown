---
name: element-classification
---
Accurately identify and classify HTML element types

Key source files:
- crates/html-to-markdown/src/visitor.rs (NodeType enum)
- crates/html-to-markdown/src/options.rs

Master concepts:
- Block vs inline categorization
- Semantic HTML5 elements
- Void element identification
- Form control recognition

Step by step:
1. Extract element tag name
2. Normalize to lowercase
3. Check against element classification tables
4. Determine category
   - Block elements: div, p, h1-h6, ul, ol, table, section
   - Inline elements: span, strong, em, code, a, img
   - Void elements: br, img, input, hr
   - Semantic elements: article, section, nav
5. Select appropriate conversion handler
