---
name: metadata-security-handler
description: Safely handle metadata and structured data
---

Key concepts:

- Meta tag handling
- JSON-LD sanitization
- Comment preservation
- Information leakage prevention

Capabilities:

- Remove or whitelist meta tags
- Handle structured data safely
- Remove comments by default
- Prevent information leakage
- Configure preservation options
- Log sensitive data detected

Patterns:

- Remove: meta, link, base
- Safe: OG/Twitter cards (if configured)
- Remove: script type=application/ld+json
- Strip comments by default
