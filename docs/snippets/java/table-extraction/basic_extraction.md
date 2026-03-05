```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.TableExtractionResult;

String html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
""";

TableExtractionResult result = HtmlToMarkdown.convertWithTables(html);

for (var table : result.tables()) {
    for (int i = 0; i < table.cells().size(); i++) {
        String prefix = table.isHeaderRow().get(i) ? "Header" : "Row";
        System.out.printf("  %s: %s%n", prefix, table.cells().get(i));
    }
}
```
