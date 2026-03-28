---
name: Meta Tag Removal
priority: high
---

Remove metadata that could expose information

- Remove all <meta> tags by default
- Remove <meta http-equiv> (potential security bypass)
- Remove <meta name> (could leak information)
- Allow safe OG/Twitter card metadata if configured
- Don't preserve referrer, robots, tracking metadata
