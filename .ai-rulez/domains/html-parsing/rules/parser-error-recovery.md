---
name: Parser Error Recovery
priority: high
---

Recover gracefully from parse errors

- Capture and report parser errors
- Continue with partial parse if recoverable
- Include error location (line, column) in errors
- Suggest fixes for common errors (unclosed tags)
- Return best-effort result even on errors
- Log detailed error information for debugging
