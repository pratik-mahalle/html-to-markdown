---
name: Visitor Method Coverage
priority: high
---
Implement visitor methods for all elements

- Visitor trait covers 60+ HTML element types
- Each element has visit_* method
- Support both pre and post hooks (visit/leave)
- Return VisitResult enum (Process, Skip, Custom)
- Maintain proper depth tracking
- Allow element modification via visitor
