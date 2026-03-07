---
name: Efficient Node Iteration
priority: high
---
Provide memory-efficient node iteration

- Implement iterator trait for child nodes
- Support filtering iterators (skip comments, text-only, elements)
- Avoid materializing entire child list in memory
- Use lazy evaluation where possible
- Document iteration order (document order guaranteed)
- Test with large child lists (1000+ children)
