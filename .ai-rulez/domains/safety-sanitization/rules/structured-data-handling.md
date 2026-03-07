---
name: Structured Data Handling
priority: high
---
Handle JSON-LD and Schema.org safely

- Remove <script type="application/ld+json"> by default
- Structured data can contain XSS payloads
- Option to whitelist specific fields if needed
- Test JSON-LD injection payloads
- Log if structured data detected
