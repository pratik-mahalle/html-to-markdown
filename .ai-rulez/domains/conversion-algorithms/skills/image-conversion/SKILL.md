---
description: "Image Conversion"
name: image-conversion
---

Convert HTML images to Markdown format

Key source files:

- crates/html-to-markdown/src/converter.rs (convert_image)

Master concepts:

- src extraction
- alt text handling
- Responsive images (srcset)
- Title attribute

Step by step:

1. Extract src attribute
2. Sanitize URL
3. Extract alt text or use filename if no alt
4. Check for title attribute
5. Format as ![alt] with src in parentheses
6. Handle responsive images
   - Check srcset attribute
   - Use best-fit source
7. Handle data URIs if configured
