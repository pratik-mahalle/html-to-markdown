```python
from html_to_markdown import ConversionOptions, convert

options = ConversionOptions(
    extract_metadata=True,
    extract_headers=True,
    extract_links=True,
    extract_images=True,
    extract_structured_data=True,
    max_structured_data_size=100000,
)
result = convert(html, options)
markdown = result["content"]
metadata = result["metadata"]
```
