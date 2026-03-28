---
name: Event Handler Prevention
priority: high
---

Block JavaScript execution via event handlers

- Remove all on* attributes
- Prevent SVG onload handlers
- Prevent image onerror handlers
- Prevent style event binding
- Verify removal in test suite
