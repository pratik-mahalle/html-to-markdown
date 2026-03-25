---
description: "Event Handler Removal"
name: event-handler-removal
---
Remove JavaScript event handlers from HTML

Key source files:
- ammonia crate configuration

Master concepts:
- Event handler attributes
- Wildcard pattern matching
- SVG event handlers
- Complete removal

Step by step:
1. Identify event handler attributes
   - HTML events like onclick, onload, onerror, onmouseover, etc.
   - SVG events like onload, onactivate, onbuild, etc.
2. Pattern match for on prefix (case-insensitive)
3. Remove all matching attributes
4. Test with encoded variants
   - HTML entity encoded (numeric and named)
   - Encoded with newlines
5. Verify removal in output
6. Log removal events
