/*
 * Basic HTML to Markdown conversion example.
 *
 * Compile:
 *   make basic_convert
 *   # or:
 *   cc -o basic_convert basic_convert.c $(pkg-config --cflags --libs html-to-markdown)
 */

#include <html_to_markdown.h>
#include <stdio.h>

int main(void) {
    printf("html-to-markdown-ffi version: %s\n\n", html_to_markdown_version());

    const char *html = "<h1>Hello World</h1>"
                       "<p>This is a <strong>bold</strong> and <em>italic</em> example.</p>"
                       "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>";

    char *md = html_to_markdown_convert(html);
    if (md) {
        printf("Markdown output:\n%s\n", md);
        html_to_markdown_free_string(md);
    } else {
        fprintf(stderr, "Error: %s\n", html_to_markdown_last_error());
        return 1;
    }

    return 0;
}
