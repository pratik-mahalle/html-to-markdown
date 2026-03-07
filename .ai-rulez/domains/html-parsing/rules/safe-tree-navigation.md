---
name: Safe Tree Navigation
priority: high
---
Provide safe, panic-free DOM tree traversal

- Implement parent(), children(), next_sibling(), prev_sibling()
- Validate pointers before dereferencing
- Return Option/Result for missing parents/siblings
- Track visited nodes to detect cycles
- Implement depth-first and breadth-first traversal
- Support filtered iteration (skip comments, text-only, etc.)
