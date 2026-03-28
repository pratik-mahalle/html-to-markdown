---
description: "Sanitization Verification"
name: sanitization-verification
---

Verify sanitization is effective

Key source files:

- Test suite

Master concepts:

- Input to Output comparison
- Element removal verification
- Content preservation
- Edge case validation

Step by step:

1. For each XSS payload:
   a. Run through sanitizer
   b. Verify dangerous elements removed
   c. Verify dangerous attrs removed
   d. Verify content preserved
2. Test edge cases:
   - Empty elements
   - Encoded attacks
   - Mixed case attacks
3. Round-trip test:
   - Input, Sanitize, Output
   - Output, Parse, Verify safe
4. Verify no regression:
   - Safe content preserved
   - Legitimate attributes kept
   - Links still work
5. Performance check:
   - Sanitization time < 10ms
