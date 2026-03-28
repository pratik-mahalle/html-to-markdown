---
name: Image Conversion Format
priority: high
---

Convert images to Markdown ![alt](src) format

- Extract src and alt text, convert to ![alt](src)
- Preserve alt text exactly
- Optional: Include title attribute
- Handle responsive images (srcset attribute)
- Fallback for images without alt (use filename)
- Sanitize image URLs
- Support data URIs if configured (inline-images feature)
- Test with various image formats
