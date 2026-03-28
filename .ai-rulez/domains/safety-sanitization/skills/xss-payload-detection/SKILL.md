---
description: "Xss Payload Detection"
name: xss-payload-detection
---

Detect and prevent XSS attack patterns

Key source files:

- OWASP XSS cheat sheets

Master concepts:

- Event handler patterns
- JavaScript URL detection
- SVG attack vectors
- CSS injection patterns

Step by step:

1. Detect event handlers
   - Pattern: on* attributes like onclick and onerror
   - Remove all on* attributes
2. Detect javascript URLs
   - Pattern: javascript at scheme start
   - Case-insensitive check
   - URL-encoded bypass check
3. Detect data URIs
   - Pattern: data with script content
   - Block or sanitize
4. Detect SVG XSS
   - SVG with script tags
   - SVG with event handlers
5. Detect CSS XSS
   - expression() in CSS
   - behavior in CSS
   - javascript URLs in CSS
6. Test against OWASP cheat sheets
