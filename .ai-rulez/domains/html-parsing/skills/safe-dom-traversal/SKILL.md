---
description: "Safe Dom Traversal"
name: safe-dom-traversal
---
Navigate DOM tree safely without panics or crashes

Key source files:
- crates/html-to-markdown/src/visitor.rs
- crates/html-to-markdown/src/visitor_helpers.rs

Master concepts:
- Parent-child-sibling navigation
- Depth-first traversal
- Visitor pattern implementation
- Depth limit enforcement

Step by step:
1. Start from document root
2. Implement visitor trait for element processing
3. Track depth to enforce limits
4. Navigate to children recursively
5. Process element content
6. Handle void elements (no children)
7. Return from leaf nodes
8. Validate all pointer operations
