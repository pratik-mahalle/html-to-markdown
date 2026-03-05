```elixir
html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

{:ok, content, tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)

for %{cells: cells, is_header_row: is_header_row} <- tables do
  cells
  |> Enum.with_index()
  |> Enum.each(fn {row, i} ->
    prefix = if Enum.at(is_header_row, i), do: "Header", else: "Row"
    IO.puts("  #{prefix}: #{Enum.join(row, ", ")}")
  end)
end
```
