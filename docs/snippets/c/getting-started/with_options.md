```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    const char *html = "<h1>Title</h1><p>Paragraph</p>";

    char *markdown = html_to_markdown_convert_to_string_with_len(
        html, strlen(html));
    if (markdown) {
        printf("%s\n", markdown);
        html_to_markdown_free_string(markdown);
    }
    return 0;
}
```
