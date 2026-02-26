/*
 * Metadata extraction example for html-to-markdown-ffi.
 *
 * Demonstrates extracting document metadata (title, description, OpenGraph)
 * alongside the markdown conversion.
 *
 * Compile:
 *   make metadata_extraction
 *   # or:
 *   cc -o metadata_extraction metadata_extraction.c $(pkg-config --cflags --libs html-to-markdown)
 */

#include <html_to_markdown.h>
#include <stdio.h>

int main(void) {
    printf("html-to-markdown metadata extraction example\n\n");

    const char *html =
        "<!DOCTYPE html>"
        "<html lang=\"en\">"
        "<head>"
        "  <title>My Page Title</title>"
        "  <meta name=\"description\" content=\"A sample web page for testing\">"
        "  <meta property=\"og:title\" content=\"OG Title\">"
        "  <meta property=\"og:description\" content=\"OG Description\">"
        "  <meta property=\"og:type\" content=\"article\">"
        "</head>"
        "<body>"
        "  <h1>Welcome</h1>"
        "  <p>This is the <strong>main content</strong> of the page.</p>"
        "  <a href=\"https://example.com\">Example link</a>"
        "</body>"
        "</html>";

    char *metadata_json = NULL;
    size_t md_len = 0;
    size_t meta_len = 0;

    char *md = html_to_markdown_convert_with_metadata_with_len(
        html, &metadata_json, &md_len, &meta_len);

    if (md && metadata_json) {
        printf("Markdown (%zu bytes):\n%s\n", md_len, md);
        printf("Metadata JSON (%zu bytes):\n%s\n", meta_len, metadata_json);
        html_to_markdown_free_string(md);
        html_to_markdown_free_string(metadata_json);
    } else {
        fprintf(stderr, "Error: %s\n", html_to_markdown_last_error());
        return 1;
    }

    return 0;
}
