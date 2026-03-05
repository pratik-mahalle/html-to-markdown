```c
#include "html_to_markdown.h"
#include <stdio.h>

const char* html =
    "<table>"
    "<tr><th>Name</th><th>Age</th></tr>"
    "<tr><td>Alice</td><td>30</td></tr>"
    "<tr><td>Bob</td><td>25</td></tr>"
    "</table>";

/* Returns JSON: {"content":"...","metadata":...,"tables":[...]} */
char* json = html_to_markdown_convert_with_tables(html, NULL, NULL);
if (json != NULL) {
    printf("%s\n", json);
    html_to_markdown_free_string(json);
}
```
