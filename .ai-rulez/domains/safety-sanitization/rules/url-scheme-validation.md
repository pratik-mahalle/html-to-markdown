---
name: URL Scheme Validation
priority: high
---

Validate URLs before including in output

- Implement sanitize_url(url, whitelist) function
- Whitelist safe schemes: http, https, mailto, ftp, tel, sms, geo
- Block dangerous schemes: javascript, data, vbscript, file
- Support relative URLs (/, ./, ../, #)
- Decode URL-encoded payloads and validate
- Handle case-insensitive scheme names
- Return None for invalid URLs
