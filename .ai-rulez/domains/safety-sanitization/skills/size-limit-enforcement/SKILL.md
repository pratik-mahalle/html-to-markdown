---
name: size-limit-enforcement
---
Enforce memory and size limits

Key source files:
- crates/html-to-markdown/src/lib.rs

Master concepts:
- Document size validation
- Nesting depth limits
- Output size limits
- Graceful rejection

Step by step:
1. Check document size
   - Default limit 50MB
   - Configurable via SafetyConfig
   - Return error if exceeded
2. Check nesting depth
   - Default limit 256 levels
   - Track during parsing
   - Return error before stack overflow
3. Check output size
   - Default limit 100MB
   - Monitor during conversion
   - Truncate or error if exceeded
4. Include limits in error message
5. Log size violation events
