---
name: Safety Integration with Parsing
priority: high
---
Ensure parsing uses validated input

- Run validate_input() before parsing
- Run sanitize_html() before parsing
- Never parse untrusted HTML directly
- Apply in correct order: validate, sanitize, parse
- Check all integration points
