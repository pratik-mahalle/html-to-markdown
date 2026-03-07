---
name: Default Safe Configuration
priority: high
---
Defaults prioritize security over functionality

- sanitize_html: true (enabled by default)
- max_document_size: 50MB (reasonable limit)
- max_nesting_depth: 256 (stack safety)
- Whitelist conservative element/attribute sets
- Block dangerous URL schemes by default
- Strip SVG by default (configurable)
- Strip comments by default
- Enable strict mode by default
