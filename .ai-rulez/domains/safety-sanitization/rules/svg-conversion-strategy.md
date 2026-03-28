---
name: SVG Conversion Strategy
priority: high
---

Define how SVG is handled in conversion

- Option 1: Extract text content from SVG
- Option 2: Use alt text or title attribute
- Option 3: Replace with [SVG: description] notation
- Option 4: Strip SVG entirely (configurable)
- Make strategy configurable via SafetyConfig
- Preserve content when extracting
- Test with complex SVG
