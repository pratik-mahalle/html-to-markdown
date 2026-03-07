---
name: Memory Efficiency
priority: high
---
Keep parser memory usage bounded

- Enforce maximum document size (default 50MB)
- Monitor memory allocation during parsing
- Use reference counting (Rc/Arc) for shared subtrees
- Avoid unnecessary node copying
- Implement memory pooling for temporary allocations
- Benchmark memory usage on large documents
