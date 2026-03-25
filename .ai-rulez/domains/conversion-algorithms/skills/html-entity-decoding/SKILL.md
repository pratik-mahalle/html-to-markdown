---
description: "Html Entity Decoding"
name: html-entity-decoding
---
Decode HTML entities in text

Key source files:
- crates/html-to-markdown/src/text.rs

Master concepts:
- Named entities (&nbsp;, &lt;, &amp;)
- Numeric entities (&#123;, &#x7B;)
- Partial entity handling
- Preservation of decoded text

Step by step:
1. Identify HTML entities
   - Named entities like &name;
   - Decimal entities like &#123;
   - Hex entities like &#x7B;
2. Decode each entity
   - &nbsp; becomes a space (non-breaking space)
   - &lt; becomes <
   - &gt; becomes >
   - &amp; becomes &
   - &#123; becomes {
3. Handle partial entities
   - Preserve incomplete entities like & nbsp
   - Preserve invalid entities like &123
4. Don't re-encode after decoding
