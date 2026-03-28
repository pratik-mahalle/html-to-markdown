---
description: "Url Sanitization"
name: url-sanitization
---

Validate and sanitize URLs in links/images

Key source files:

- crates/html-to-markdown/src/safety.rs

Master concepts:

- Scheme validation
- Dangerous URL detection
- Safe URL formats

Step by step:

1. Extract URL from href or src
2. Check for dangerous schemes
   - Block javascript, data, vbscript
3. Validate safe schemes
   - Allow http, https, mailto, ftp, /, #
4. For relative URLs pass through
5. For absolute URLs validate scheme
6. Return sanitized URL or None
