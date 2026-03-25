---
description: "Parser Error Handling"
name: parser-error-handling
---
Handle parser errors gracefully with recovery

Key source files:
- crates/html-to-markdown/src/error.rs

Master concepts:
- Error types and classification
- Partial parsing recovery
- Error context preservation
- User-friendly messages

Step by step:
1. Wrap parser call in Result
2. Catch parser errors
3. Classify error type
   - Malformed HTML
   - Size limit exceeded
   - Nesting depth exceeded
   - Encoding error
4. If recoverable
   a. Attempt partial parse
   b. Continue with best-effort result
5. Include error context
   - Input location
   - Error description
   - Remediation hint
6. Return Err(ConversionError)
