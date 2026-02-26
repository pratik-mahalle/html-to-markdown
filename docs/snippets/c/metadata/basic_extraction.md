```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    const char *html = "<html><head><title>Page</title></head>"
                       "<body><h1>Hello</h1></body></html>";

    char *result_json = html_to_markdown_convert_with_metadata_with_len(
        html, strlen(html), NULL, 0);
    if (result_json) {
        /* result_json is a JSON string with "markdown" and "metadata" fields */
        printf("%s\n", result_json);
        html_to_markdown_free_string(result_json);
    }
    return 0;
}
```
