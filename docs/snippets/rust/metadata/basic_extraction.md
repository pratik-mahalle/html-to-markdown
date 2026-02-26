```rust
use html_to_markdown_rs::convert_with_metadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"<html><head><title>My Page</title></head>
    <body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>"#;

    let result = convert_with_metadata(html, None, None)?;
    println!("Markdown: {}", result.markdown);
    println!("Title: {:?}", result.metadata.title);
    println!("Links: {:?}", result.metadata.links);
    Ok(())
}
```
