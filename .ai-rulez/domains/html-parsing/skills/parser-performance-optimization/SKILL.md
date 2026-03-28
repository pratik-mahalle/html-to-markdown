---
description: "Parser Performance Optimization"
name: parser-performance-optimization
---

Optimize parser speed and memory usage

Key source files:

- crates/html-to-markdown/src/converter.rs

Master concepts:

- Parser benchmarking
- Memory allocation patterns
- Hot path optimization
- Caching strategies

Step by step:

1. Benchmark current parser
2. Profile with flamegraph
3. Identify hot paths
4. Optimize code
   a. Reduce string copies
   b. Use reference counting
   c. Implement node caching
   d. Lazy evaluation
5. Verify correctness after optimization
6. Benchmark improvement
