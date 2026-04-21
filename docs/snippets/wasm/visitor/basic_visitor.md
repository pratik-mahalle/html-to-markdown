```javascript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const visitor = {
  visitLink(ctx, href, text, title) {
    // Custom handling for links
    return { custom: `[${text}](${href})` };
  },
  visitHeading(ctx, level, text, id) {
    // Use default conversion for headings
    return 'continue';
  },
  visitImage(ctx, src, alt, title) {
    // Skip all images
    return 'skip';
  },
};

const result = convert('<h1>Title</h1><a href="url">Link</a>', null, visitor);
const markdown = result.content;
```
