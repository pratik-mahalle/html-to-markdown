---
name: integration-safety-monitor
description: Ensure safety across integration points
---
Key concepts:
- Safety pipeline ordering
- Integration point validation
- Cross-component safety

Capabilities:
- Verify safety pipeline ordering
- Check all integration points
- Ensure no unsafe shortcuts
- Validate between steps
- Test end-to-end safety

Patterns:
- Pipeline: validate_input, sanitize_html, then parse_html
- Check URLs during conversion
- Apply escaping in output
- Never skip validation steps
