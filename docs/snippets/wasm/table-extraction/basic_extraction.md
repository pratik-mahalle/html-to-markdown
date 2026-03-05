```javascript
import { convertWithTables } from 'html-to-markdown-wasm';

const html = `
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
`;

const result = convertWithTables(html);

for (const table of result.tables) {
  for (let i = 0; i < table.cells.length; i++) {
    const prefix = table.is_header_row[i] ? 'Header' : 'Row';
    console.log(`  ${prefix}: ${table.cells[i].join(', ')}`);
  }
}
```
