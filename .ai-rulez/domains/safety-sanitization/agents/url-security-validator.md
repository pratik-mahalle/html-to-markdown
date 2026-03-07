---
name: url-security-validator
description: Validate and sanitize URLs in href and src attributes
---
Source: crates/html-to-markdown/src/safety.rs

Key concepts:
- URL scheme validation
- JavaScript URL detection
- Data URI handling
- Protocol whitelist enforcement

Capabilities:
- Validate URL schemes against whitelist
- Detect javascript: URLs (case-insensitive)
- Detect data: URIs with dangerous content
- Handle URL-encoded payloads
- Support relative URLs
- Detect unicode tricks
- Return sanitized URL or None

Patterns:
- Whitelist: http, https, mailto, ftp, tel, /
- Block: javascript, data, vbscript, file
- Decode and validate URL-encoded input
- Relative URLs: /, ./, #
