```javascript
import init, { convertWithMetadata } from "@kreuzberg/html-to-markdown-wasm";

await init();

const html = '<html><head><title>My Page</title></head><body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>';
const result = convertWithMetadata(html);

console.log("Markdown:", result.markdown);
console.log("Title:", result.metadata.title);
console.log("Links:", result.metadata.links);
```
