---
description: "Metadata Handling"
name: metadata-handling
---
Safely handle metadata and structured data

Key source files:
- ammonia configuration

Master concepts:
- Meta tag removal
- Structured data safety
- Comment preservation
- Information leakage prevention

Step by step:
1. Meta tags removed by default
   - Remove all meta tags
   - Prevents http-equiv and tracking
2. Structured data handling
   - Remove script tags with application/ld+json
   - JSON-LD can contain XSS
3. Comments
   - Remove by default
   - Prevent exposing commented code
4. Optional whitelist of safe metadata
   - OG tags (if configured)
   - Twitter cards (if configured)
5. Log if sensitive data detected
