---
name: security-error-recovery
description: Handle security errors gracefully
---

Key concepts:

- Graceful degradation
- Error context preservation
- User-friendly error messages
- Partial content recovery

Capabilities:

- Catch sanitization errors
- Continue with safe output
- Provide detailed error info
- Log security events
- Return best-effort result
- Never expose dangerous content

Patterns:

- Return error or safe fallback
- Preserve safe content
- Clear error messages
- Log for audit trail
