---
name: Panic Safety
priority: high
---

Ensure sanitization never panics

- Use Result/Option instead of unwrap/expect
- Catch parsing panics with guard_panic()
- Return errors instead of panicking
- Test sanitization with fuzzing
- Ensure safety across all code paths
