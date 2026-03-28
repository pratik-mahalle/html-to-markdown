---
name: Size Limit Enforcement
priority: high
---

Prevent unbounded memory allocation

- Enforce maximum document size (default 50MB)
- Enforce maximum nesting depth (default 256 levels)
- Enforce maximum output size (default 100MB)
- Check limits before parsing
- Return error if limits exceeded
- Make limits configurable via SafetyConfig
- Include limits in error messages
