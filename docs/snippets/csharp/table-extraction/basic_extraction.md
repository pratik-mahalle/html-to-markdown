```csharp
using HtmlToMarkdown;

var html = @"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>";

var options = new ConversionOptions { ExtractTables = true };
var result = HtmlToMarkdownConverter.Convert(html, options);

foreach (var table in result.Tables ?? [])
{
    for (int i = 0; i < table.Cells.Count; i++)
    {
        var prefix = table.IsHeaderRow[i] ? "Header" : "Row";
        Console.WriteLine($"  {prefix}: {string.Join(", ", table.Cells[i])}");
    }
}
```
