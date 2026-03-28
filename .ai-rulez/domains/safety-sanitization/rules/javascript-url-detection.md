---
name: JavaScript URL Detection
priority: high
---

Detect and block javascript: URLs

- Check for javascript prefix (case-insensitive)
- Detect URL-encoded javascript patterns
- Detect whitespace variations in URL schemes
- Return None if detected
- Test with OWASP URL evasion examples
