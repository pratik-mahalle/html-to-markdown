---
name: Event Handler Removal
priority: high
---

Remove all JavaScript event handlers

- Remove all attributes matching /^on.*/i
- Remove onclick, onload, onerror, onmouseover, etc.
- Check both HTML and SVG event handlers
- Verify removal in sanitized output
- Test with encoded event handlers (if applicable)
