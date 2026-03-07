---
name: xss-test-implementation
---
Create comprehensive XSS test suite

Key source files:
- OWASP XSS Filter Evasion
- HTML5 Security Cheat Sheet

Master concepts:
- OWASP test cases
- HTML5 attack vectors
- Browser-specific XSS
- Encoding/obfuscation bypasses

Step by step:
1. Implement OWASP XSS tests
   - Basic alerts with script tags
   - Event handlers like onerror
   - URL injection attacks
2. Implement HTML5 tests
   - SVG event handlers
   - Data URI payloads
3. Test encoding bypasses
   - URL encoding patterns
   - HTML entity encoding
   - Unicode escape sequences
4. Test browser-specific attacks
   - IE specific techniques
   - Firefox specific techniques
5. Verify sanitization blocks all tests
