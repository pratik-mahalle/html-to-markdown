```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionResult;

public class MetadataExample {
    public static void main(String[] args) {
        String html = """
            <html><head><title>My Page</title></head>
            <body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>
            """;

        ConversionResult result = HtmlToMarkdown.convertWithMetadata(html);
        System.out.println("Markdown: " + result.getMarkdown());
        System.out.println("Title: " + result.getMetadata().getTitle());
        System.out.println("Links: " + result.getMetadata().getLinks());
    }
}
```
