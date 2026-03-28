```c
#include "html_to_markdown.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *html = "<h1>Title</h1><p>Paragraph</p>";
    const char *options_json = "{\"heading_style\":\"atx\"}";

    /* Returns JSON: {"content":"...","metadata":null,"tables":null} */
    char *json = html_to_markdown_convert_with_len(
        html, strlen(html), options_json, strlen(options_json));
    if (json) {
        /* Parse JSON to extract content field */
        printf("%s\n", json);
        html_to_markdown_free_string(json);
    }
    return 0;
}
```
