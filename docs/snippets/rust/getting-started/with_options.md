```rust
use html_to_markdown_rs::{convert, ConversionOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConversionOptions {
        heading_style: Some("atx".into()),
        skip_images: Some(true),
        ..Default::default()
    };
    let markdown = convert("<h1>Hello</h1><img src='pic.jpg'>", Some(options))?;
    println!("{markdown}");
    Ok(())
}
```
