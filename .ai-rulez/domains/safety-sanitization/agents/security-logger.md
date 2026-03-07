---
name: security-logger
description: Log security events for audit and debugging
---
Key concepts:
- Security event types
- Audit trail creation
- Anomaly detection
- Detailed error logging

Capabilities:
- Log binary data detection
- Log sanitization events
- Log XSS attempt detection
- Log URL validation failures
- Create audit trail
- Enable detailed logging mode
- Avoid exposing sensitive data

Patterns:
- Log type, timestamp, context
- Include what was removed/blocked
- Don't expose dangerous payload
- Aggregate statistics
