---
name: Attribute Whitelisting
priority: high
---
Only allow safe attributes on elements

- Whitelist global attributes: id, class, title, lang, dir, data-*
- Whitelist element-specific: href (links), src (images), alt
- Whitelist formatting: colspan, rowspan, align (tables)
- Remove all event handlers: on*, javascript:
- Remove dangerous attributes: style (if configured), srcdoc, action, formaction
- Remove URL-based attributes on unsafe elements
- Validate attribute values after whitelisting
