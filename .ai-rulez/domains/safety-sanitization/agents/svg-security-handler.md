---
name: svg-security-handler
description: Safely handle SVG elements and prevent XSS
---
Key concepts:
- SVG script removal
- Event handler prevention
- URL validation in SVG
- Safe SVG conversion

Capabilities:
- Remove <script> within SVG
- Remove SVG event handlers
- Validate xlink:href URLs
- Extract text content from SVG
- Configure SVG handling strategy
- Fall back to alt text

Patterns:
- Strip script and style tags with JavaScript
- Remove event handlers like onload and onclick
- Validate xlink:href, href, and src attributes
- Use alt notation like [SVG: description]
