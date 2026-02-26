```rust
use html_to_markdown_rs::{convert_with_visitor, Visitor, VisitResult};

struct LinkRewriter;

impl Visitor for LinkRewriter {
    fn visit_link(&self, url: &str, text: &str) -> VisitResult {
        // Rewrite all links to use a tracking prefix
        VisitResult::Replace(format!("[{text}](https://track.example.com?url={url})"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"<a href="https://example.com">Click here</a>"#;
    let markdown = convert_with_visitor(html, LinkRewriter, None)?;
    println!("{markdown}");
    Ok(())
}
```
