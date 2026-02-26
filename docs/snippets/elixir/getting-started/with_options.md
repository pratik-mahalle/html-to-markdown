```elixir
handle = HtmlToMarkdown.options(%HtmlToMarkdown.Options{wrap: true, wrap_width: 40})
{:ok, markdown} = HtmlToMarkdown.convert_with_options("<h1>Hello</h1><p>World</p>", handle)
IO.puts(markdown)
```
