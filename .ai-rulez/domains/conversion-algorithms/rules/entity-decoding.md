---
name: Entity Decoding
priority: high
---

Decode HTML entities in text content

- Decode named entities: &nbsp; &lt; &gt; &amp; etc.
- Decode numeric entities: &#123; &#x7B; etc.
- Handle partial entities gracefully
- Preserve decoded content in output
- Don't re-encode after decoding
