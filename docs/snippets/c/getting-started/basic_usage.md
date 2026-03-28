```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    const char *html = "<h1>Hello</h1><p>World</p>";
    /* Returns JSON: {"content":"...","metadata":null,"tables":null} */
    char *json = html_to_markdown_convert(html, NULL);
    if (json) {
        /* Parse JSON to extract content field */
        printf("%s\n", json);
        html_to_markdown_free_string(json);
    }
    return 0;
}
```
