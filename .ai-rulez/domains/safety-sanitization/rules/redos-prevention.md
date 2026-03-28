---
name: ReDoS Prevention
priority: high
---

Prevent Regular Expression Denial of Service

- Avoid catastrophic backtracking in regex
- Use tested regex patterns
- Limit regex match time with timeouts if possible
- Test regex with pathological inputs
- Use regex crate with good performance
