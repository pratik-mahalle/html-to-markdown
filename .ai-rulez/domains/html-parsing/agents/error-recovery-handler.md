---
name: error-recovery-handler
description: Handle parser errors gracefully
---
Key concepts:
- Parser error types
- Partial parsing recovery
- Error reporting and logging
- Graceful degradation

Capabilities:
- Capture parser errors without panicking
- Continue with partial results when possible
- Provide detailed error context
- Suggest remediation steps
- Log errors for debugging

Patterns:
- Return Result<T, ConversionError>
- Include error location and context
- Attempt recovery when safe
- Preserve max content on error
