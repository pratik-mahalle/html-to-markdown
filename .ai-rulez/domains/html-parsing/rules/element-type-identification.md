---
name: Element Type Identification
priority: high
---
Accurately identify all HTML element types

- Support 60+ HTML5 element types
- Implement NodeType enum (Element, Text, Comment, Document)
- Provide is_element(), is_text(), is_void_element() helpers
- Map element names to semantic categories (Block, Inline, Form, etc.)
- Handle case-insensitive tag names (lowercase internally)
- Support custom element detection if configured
