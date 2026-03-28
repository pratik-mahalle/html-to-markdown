---
name: Element Whitelisting
priority: high
---

Only allow safe HTML elements

- Always remove: script, style, iframe, object, embed, applet, form
- Always remove: meta, link, base, input, button, textarea, select
- Safe block elements: p, div, h1-h6, ul, ol, li, blockquote, pre, hr
- Safe inline elements: span, strong, em, b, i, code, a, img, br
- Safe table elements: table, tr, td, th, thead, tbody, tfoot, caption
- Safe semantic: article, section, nav, aside, header, footer
- Remove unknown/custom elements
- Preserve text content when removing elements
