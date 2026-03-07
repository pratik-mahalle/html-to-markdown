---
name: Performance Optimization
priority: high
---
Maintain fast conversion speed

- Target <1ms per 1000 elements
- Profile hot paths (list/table conversion)
- Use string builders/buffers (not repeated concatenation)
- Cache frequently converted structures
- Benchmark with large documents
- Document performance characteristics
