---
description: "Ammonia Sanitization"
name: ammonia-sanitization
---

Use ammonia crate for HTML sanitization

Key source files:

- ammonia crate integration

Master concepts:

- Whitelist configuration
- Element filtering
- Attribute filtering
- URL validation

Step by step:

1. Create ammonia Builder instance
2. Configure element whitelist
   - Safe tags: p, div, h1-h6, ul, ol, li, a, img, strong, em, code
   - Unsafe tags: script, style, iframe, form, input, etc.
3. Configure attribute whitelist
   - Global attributes: id, class, title, lang, dir, data-*
   - Link attributes: href, target, rel
   - Image attributes: src, alt, width, height
   - Table attributes: colspan, rowspan, align
4. Configure URL validation
   - Set allowed schemes: http, https, mailto, ftp
   - Block dangerous schemes: javascript, data
5. Set link rel to noopener noreferrer
6. Clean HTML using cleaner.clean()
