---
name: Attribute Access & Retrieval
priority: high
---

Provide comprehensive attribute inspection

- Implement get_attribute(elem, name) → Option<String>
- Implement get_attributes(elem) → BTreeMap<String, String>
- Support case-insensitive attribute name matching
- Return URL-encoded attribute values as-is (don't decode)
- Provide helpers: has_class(), get_classes(), get_id()
- Handle boolean attributes (checked, disabled, selected)
- Validate attribute values before returning
