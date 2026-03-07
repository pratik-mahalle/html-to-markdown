---
name: SVG Script Prevention
priority: high
---
Remove executable content from SVG

- Remove <script> tags within SVG
- Remove <style> with JavaScript bindings
- Remove event handler attributes (onload, onclick, etc.)
- Remove animation with malicious behavior
- Make SVG handling configurable (preserve, sanitize, or strip)
- Test SVG XSS payloads
