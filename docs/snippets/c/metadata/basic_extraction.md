```c
#include "html_to_markdown.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    const char *html = "<html><head><title>Page</title></head>"
                       "<body><h1>Hello</h1></body></html>";
    const char *options_json = "{\"extract_metadata\":true}";

    /* Returns JSON: {"content":"...","metadata":{...},"tables":null} */
    char *json = html_to_markdown_convert_with_len(
        html, strlen(html), options_json, strlen(options_json));
    if (json) {
        /* Parse JSON to access content and metadata fields */
        printf("%s\n", json);
        html_to_markdown_free_string(json);
    }
    return 0;
}
```
