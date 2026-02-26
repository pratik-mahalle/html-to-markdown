```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.metadata.MetadataExtraction;

public class MetadataExample {
    public static void main(String[] args) {
        String html = "<html><head><title>My Page</title></head>"
            + "<body><h1>Welcome</h1><a href=\"https://example.com\">Link</a></body></html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        System.out.println("Markdown: " + result.markdown());
        System.out.println("Title: " + result.metadata().document().title());
        System.out.println("Headers: " + result.metadata().headers().size());
        System.out.println("Links: " + result.metadata().links().size());
    }
}
```
