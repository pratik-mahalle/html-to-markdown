```rust
use html_to_markdown_rs::convert_with_tables;

let html = r#"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"#;

let result = convert_with_tables(html, None, None)?;

for table in &result.tables {
    for (i, row) in table.cells.iter().enumerate() {
        let prefix = if table.is_header_row[i] { "Header" } else { "Row" };
        println!("  {prefix}: {}", row.join(", "));
    }
}
```
