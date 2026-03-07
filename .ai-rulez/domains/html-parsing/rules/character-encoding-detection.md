---
name: Character Encoding Detection
priority: high
---
Automatically detect and handle multiple character encodings

- Use encoding_rs for robust encoding detection
- Check HTML5 charset meta tag if present
- Detect BOM markers (UTF-8, UTF-16, UTF-32)
- Convert to UTF-8 before parsing
- Preserve original encoding declaration in metadata
- Fall back to UTF-8 if detection fails
- Validate UTF-8 sequence integrity
