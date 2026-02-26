```javascript
import init, { convert } from '@kreuzberg/html-to-markdown-wasm';

await init();

const markdown = convert('<h1>Hello</h1><img src="pic.jpg">', {
  headingStyle: 'atx',
  skipImages: true,
});
console.log(markdown);
```
