---
name: Parser Performance
priority: high
---
Maintain fast parsing times

- Target <5ms parsing time for 100KB documents
- Profile parser with flamegraphs
- Optimize hot paths (child iteration, attribute lookup)
- Consider LRU cache for frequently accessed nodes
- Use zero-copy parsing where possible
- Document performance characteristics
