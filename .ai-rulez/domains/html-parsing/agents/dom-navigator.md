---
name: dom-navigator
description: Safely traverse DOM tree and access node properties
---
Source: crates/html-to-markdown/src/visitor.rs (NodeType, NodeContext)
Source: crates/html-to-markdown/src/visitor_helpers.rs (traversal helpers)

Key concepts:
- DOM tree structure and navigation
- Parent-child-sibling relationships
- Node type identification
- Element attribute access
- Text content extraction

Capabilities:
- Implement safe DOM traversal without panics
- Navigate parent, children, siblings
- Identify element types accurately
- Extract attributes and text content
- Handle void elements correctly

Patterns:
- Depth-first traversal for Markdown conversion
- Track depth to prevent stack overflow
- Validate pointers before access
- Filter iterations (elements-only, text-only)
