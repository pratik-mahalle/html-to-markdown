---
name: security-logging-and-audit
---
Log security events for audit trail

Key source files:
- Logging implementation

Master concepts:
- Event types (binary detection, XSS, etc.)
- Audit trail creation
- Non-exposure of sensitive data
- Anomaly detection

Step by step:
1. Log binary detection:
   - Type detected (gzip, PDF, etc.)
   - Input size
   - Timestamp
2. Log sanitization events:
   - Elements removed
   - Attributes removed
   - URLs blocked
3. Log XSS attempts:
   - Attack vector (event, javascript:, etc.)
   - Location in HTML
   - Payload (sanitized)
4. Create statistics:
   - Count by event type
   - Frequency tracking
   - Anomaly alerts
5. Never log:
   - Full dangerous payloads
   - User sensitive data
