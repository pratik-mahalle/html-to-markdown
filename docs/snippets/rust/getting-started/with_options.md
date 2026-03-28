```rust
use html_to_markdown_rs::{convert, ConversionOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ConversionOptions::builder()
        .heading_style(HeadingStyle::Atx)
        .skip_images(true)
        .build();
    let result = convert("<h1>Hello</h1><img src='pic.jpg'>", Some(options))?;
    let markdown = result.content.unwrap_or_default();
    println!("{markdown}");
    Ok(())
}
```
