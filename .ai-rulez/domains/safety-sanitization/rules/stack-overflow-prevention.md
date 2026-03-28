---
name: Stack Overflow Prevention
priority: high
---

Prevent stack overflow from deep recursion

- Track nesting depth during traversal
- Enforce maximum depth (default 256 levels)
- Return error before stack exhaustion
- Test with pathological nested HTML
- Make depth limit configurable
