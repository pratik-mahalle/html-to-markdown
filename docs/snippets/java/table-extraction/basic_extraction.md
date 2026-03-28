```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.ConversionResult;

String html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
""";

ConversionOptions options = ConversionOptions.builder()
    .extractTables(true)
    .build();
ConversionResult result = HtmlToMarkdown.convert(html, options);

for (var table : result.tables()) {
    for (int i = 0; i < table.cells().size(); i++) {
        String prefix = table.isHeaderRow().get(i) ? "Header" : "Row";
        System.out.printf("  %s: %s%n", prefix, table.cells().get(i));
    }
}
```
