```csharp
using HtmlToMarkdown;

var html = "<h1>Hello World</h1><p>This is a paragraph.</p>";
var markdown = HtmlToMarkdownConverter.Convert(html);
Console.WriteLine(markdown);
```
