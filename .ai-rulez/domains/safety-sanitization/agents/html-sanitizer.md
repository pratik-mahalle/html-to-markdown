---
name: html-sanitizer
description: Remove dangerous HTML elements and attributes
---

Source: ammonia crate integration

Key concepts:

- Element whitelisting
- Attribute whitelisting
- Event handler removal
- Style sanitization

Capabilities:

- Configure ammonia for HTML sanitization
- Define element whitelist
- Define attribute whitelist
- Remove event handlers
- Handle style attributes
- Preserve safe content
- Apply URL validation

Patterns:

- Remove dangerous elements: script, style, iframe, object, form, etc.
- Whitelist safe elements: p, div, a, img, strong, em, etc.
- Remove event handlers and dangerous URL schemes
- Preserve text content of removed elements
