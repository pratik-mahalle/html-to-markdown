---
name: Style Attribute Handling
priority: high
---
Sanitize or remove inline styles safely

- Option 1: Remove style attribute entirely (safest)
- Option 2: Sanitize CSS (remove expression, behavior, javascript:)
- Option 3: Preserve safe styles only (color, font, text-align)
- Make strategy configurable via SafetyConfig
- Validate CSS properties if preserving
- Test with CSS injection payloads
- Default to removal for maximum safety
