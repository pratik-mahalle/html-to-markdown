---
name: Data URI Handling
priority: high
---

Handle data: URIs safely

- Option 1: Block all data: URIs (safest)
- Option 2: Allow only safe MIME types (image/*, text/plain)
- Option 3: Validate base64 encoding
- Make strategy configurable
- Detect data: URIs with malicious content (javascript)
- Default to blocking for security
