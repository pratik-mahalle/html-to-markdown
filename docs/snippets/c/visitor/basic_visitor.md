```c
#include "html_to_markdown.h"
#include <stdio.h>
#include <string.h>

/* Custom heading visitor: prefix all headings with a section marker */
static HtmlToMarkdownVisitResult visit_heading(
    const HtmlToMarkdownVisitHeadingData *data, void *user_data) {
    (void)user_data;
    HtmlToMarkdownVisitResult result = {0};
    /* Use default conversion for all headings */
    result.type = Continue;
    return result;
}

int main(void) {
    const char *html = "<h1>Title</h1><p>Content</p>";

    html_to_markdown_visitor_callbacks_t callbacks = {0};
    callbacks.visit_heading = (struct Option_HtmlToMarkdownVisitHeadingCallback){
        .is_some = true,
        .value = visit_heading,
    };

    HtmlToMarkdownVisitor *visitor = html_to_markdown_visitor_new(&callbacks);
    char *markdown = html_to_markdown_convert_with_visitor(html, visitor);
    if (markdown) {
        printf("%s\n", markdown);
        html_to_markdown_free_string(markdown);
    }
    html_to_markdown_visitor_free(visitor);
    return 0;
}
```
