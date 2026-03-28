```python
from html_to_markdown import ConversionOptions, convert

html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

options = ConversionOptions(extract_tables=True)
result = convert(html, options)

for table in result["tables"]:
    for i, row in enumerate(table["cells"]):
        prefix = "Header" if table["is_header_row"][i] else "Row"
        print(f"  {prefix}: {row}")
```
