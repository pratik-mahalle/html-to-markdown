---
name: attribute-accessor
description: Safely access and validate element attributes
---
Key concepts:
- Case-insensitive attribute lookup
- Class and ID extraction
- URL attributes (href, src)
- Custom data attributes

Capabilities:
- Retrieve attributes by name (case-insensitive)
- Iterate all element attributes
- Extract class names and IDs
- Access URL attributes safely
- Handle boolean attributes

Patterns:
- get_attribute(elem, "href") returns Option<String>
- get_classes(elem) returns Vec<String>
- get_id(elem) returns Option<String>
- has_class(elem, "class-name") returns bool
