---
description: "Svg Security Hardening"
name: svg-security-hardening
---

Safely handle SVG elements

Key source files:

- SVG security considerations

Master concepts:

- SVG script removal
- SVG event handler blocking
- Safe SVG features
- Text extraction

Step by step:

1. Detect SVG elements
2. Remove dangerous content
   - script tags within SVG
   - style tags with JavaScript
   - Event handlers like onload and onclick
3. Validate URL attributes
   - xlink:href needs URL validation
   - href in SVG needs URL validation
   - image src needs URL validation
4. Options for SVG handling
   a. Strip SVG entirely (safest)
   b. Remove scripts and handlers (semi-safe)
   c. Extract text content
5. Fallback using alt text or title
