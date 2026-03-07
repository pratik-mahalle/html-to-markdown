---
name: Whitespace Mode Configuration
priority: high
---
Support multiple whitespace handling strategies

- Implement WhitespaceMode enum: Preserve, Minimal, Collapse
- Preserve mode: Keep all whitespace as-is
- Minimal mode: Remove leading/trailing, normalize internal
- Collapse mode: Collapse sequences to single space (HTML5 default)
- Make mode configurable per document
- Document mode behavior clearly
- Test each mode with edge cases
