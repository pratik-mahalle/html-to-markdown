```python
from html_to_markdown import convert_with_tables

html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

result = convert_with_tables(html)

for table in result["tables"]:
    for i, row in enumerate(table["cells"]):
        prefix = "Header" if table["is_header_row"][i] else "Row"
        print(f"  {prefix}: {row}")
```
