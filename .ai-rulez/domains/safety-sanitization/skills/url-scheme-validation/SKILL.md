---
description: "Url Scheme Validation"
name: url-scheme-validation
---
Validate and sanitize URLs

Key source files:
- crates/html-to-markdown/src/safety.rs

Master concepts:
- Scheme extraction
- Whitelist enforcement
- Dangerous scheme blocking
- URL-encoded bypass detection

Step by step:
1. Extract URL from attribute
2. Trim and lowercase for checking
3. Check for dangerous schemes
   - javascript (case-insensitive)
   - data (if blocking)
   - vbscript
   - file
4. Check for encoded bypasses
   - URL-encoded javascript patterns
   - Decode and check again
5. If dangerous detected return None
6. Validate against whitelist
   - Safe schemes include http, https, mailto, ftp, tel, sms
   - Relative URLs include /, ./, #
7. Return sanitized URL or None
