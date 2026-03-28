```c
#include "html_to_markdown.h"
#include <stdio.h>

const char* html =
    "<table>"
    "<tr><th>Name</th><th>Age</th></tr>"
    "<tr><td>Alice</td><td>30</td></tr>"
    "<tr><td>Bob</td><td>25</td></tr>"
    "</table>";

const char* options_json = "{\"extract_tables\":true}";

/* Returns JSON: {"content":"...","metadata":null,"tables":[...]} */
char* json = html_to_markdown_convert(html, options_json);
if (json != NULL) {
    /* Parse JSON to access tables array */
    printf("%s\n", json);
    html_to_markdown_free_string(json);
}
```
