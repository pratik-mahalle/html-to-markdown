---
description: "Attribute Access And Validation"
name: attribute-access-and-validation
---
Safely access and retrieve element attributes

Key source files:
- crates/html-to-markdown/src/visitor.rs (NodeContext)

Master concepts:
- Case-insensitive attribute lookup
- Class and ID extraction
- URL attributes (href, src, srcset)
- Boolean attribute handling

Step by step:
1. Get attribute map from element
2. For specific attribute lookup
   a. Lookup by name (case-insensitive)
   b. Return value if present
   c. Return None if absent
3. For class attribute
   a. Split by whitespace
   b. Return vector of classes
4. For ID attribute
   a. Return single ID value
5. For URL attributes
   a. Extract value
   b. Pass to URL sanitizer
