---
name: xss-prevention-specialist
description: Prevent XSS attacks across all attack vectors
---

Key concepts:

- Event handler prevention
- Dangerous attribute removal
- SVG script removal
- CSS injection prevention

Capabilities:

- Remove all on* event handler attributes
- Detect and block javascript: URLs
- Remove SVG scripts and handlers
- Sanitize CSS (or remove styles entirely)
- Test against OWASP XSS cheat sheets
- Verify XSS prevention effectiveness

Patterns:

- All on* attributes removed
- javascript: scheme blocked
- SVG <script> removed
- Event binding prevented
