```javascript
import init, { convert } from '@kreuzberg/html-to-markdown-wasm';

await init();

const html = '<h1>Hello</h1><p>This is <strong>fast</strong>!</p>';
const markdown = convert(html);
console.log(markdown);
```
