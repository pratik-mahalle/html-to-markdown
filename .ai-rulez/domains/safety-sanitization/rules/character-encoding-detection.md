---
name: Character Encoding Detection
priority: high
---

Handle multiple character encodings safely

- Detect encoding from HTML5 charset meta tag
- Check for BOM markers (UTF-8, UTF-16, UTF-32)
- Use encoding_rs for robust detection
- Convert to UTF-8 before parsing
- Validate encoding is supported
- Fall back to UTF-8 if detection fails
- Reject invalid UTF-8 sequences
