---
name: XSS Attack Vector Mitigation
priority: high
---
Prevent common XSS attack patterns

- Stored XSS: Sanitize input via ammonia
- Reflected XSS: Don't trust URL parameters
- DOM XSS: Markdown output cannot execute scripts
- CSS Injection: Validate/remove inline styles
- Test against OWASP XSS Prevention Cheat Sheet
- Test against HTML5 Security Cheat Sheet
