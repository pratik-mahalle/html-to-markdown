```c
#include "html_to_markdown.h"
#include <stdio.h>

int main(void) {
    const char *html = "<h1>Hello</h1><p>World</p>";
    char *markdown = html_to_markdown_convert(html);
    if (markdown) {
        printf("%s\n", markdown);
        html_to_markdown_free_string(markdown);
    }
    return 0;
}
```
