---
name: parser-performance-optimizer
description: Optimize parsing speed and memory usage
---

Key concepts:

- Parser benchmark metrics
- Memory allocation patterns
- Hot path optimization
- Caching opportunities

Capabilities:

- Profile parser performance
- Optimize hot paths
- Reduce unnecessary allocations
- Implement parsing caches
- Balance speed vs correctness

Patterns:

- Use reference counting for shared data
- Implement LRU cache for frequent lookups
- Minimize string copies
- Profile with flamegraphs
