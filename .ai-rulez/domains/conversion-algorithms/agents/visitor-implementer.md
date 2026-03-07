---
name: visitor-implementer
description: Implement visitor pattern for element conversion
---
Source: crates/html-to-markdown/src/visitor.rs

Key concepts:
- Visitor trait definition
- visit_* methods for elements
- Pre/post hooks
- VisitResult enum

Capabilities:
- Implement HtmlVisitor trait
- Provide visit_* methods for all element types
- Support custom visitors
- Dispatch to appropriate converters
- Track visitor state (depth, context)
- Return proper VisitResult values

Patterns:
- VisitResult::Process for default element conversion
- VisitResult::Skip to skip processing an element
- VisitResult::Custom for custom conversion handling
- Visitor composition for complex customization scenarios
