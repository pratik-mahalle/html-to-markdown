```rust
use html_to_markdown_rs::{convert, ConversionOptions};

let html = r#"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"#;

let options = ConversionOptions::builder()
    .extract_tables(true)
    .build();
let result = convert(html, Some(options))?;

for table in result.tables.unwrap_or_default() {
    for (i, row) in table.cells.iter().enumerate() {
        let prefix = if table.is_header_row[i] { "Header" } else { "Row" };
        println!("  {prefix}: {}", row.join(", "));
    }
}
```
