---
name: Depth Limit Enforcement
priority: high
---
Prevent stack overflow from deeply nested HTML

- Track nesting depth during traversal
- Enforce maximum depth (default 256 levels)
- Return error if depth exceeded
- Include depth in error message
- Make depth limit configurable via ConversionOptions
- Test with pathological nested HTML (1000+ levels)
