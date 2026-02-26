```rust
use html_to_markdown_rs::convert;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
    let markdown = convert(html, None)?;
    println!("{markdown}");
    Ok(())
}
```
