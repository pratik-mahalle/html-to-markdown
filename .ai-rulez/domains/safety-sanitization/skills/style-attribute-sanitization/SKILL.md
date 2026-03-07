---
name: style-attribute-sanitization
---
Sanitize or remove CSS in style attributes

Key source files:
- ammonia crate or custom CSS sanitization

Master concepts:
- CSS injection patterns
- Safe CSS properties
- Dangerous CSS values
- Configuration options

Step by step:
1. Decision to remove or sanitize
   - Remove is safest and simple
   - Sanitize means preserve styling
2. If removing delete all style attributes
3. If sanitizing
   a. Parse CSS properties
   b. Whitelist safe properties
      - color and background-color
      - font-size and font-weight
      - text-align and padding and border
   c. Block dangerous properties
      - expression for IE
      - behavior for IE
      - binding for Firefox
   d. Block dangerous values
      - javascript URLs
      - expression functions
      - url with dangerous schemes
4. Return sanitized style or remove
