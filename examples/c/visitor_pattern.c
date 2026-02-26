/*
 * Visitor pattern example for html-to-markdown-ffi.
 *
 * Demonstrates using callbacks to customize conversion on a per-element basis:
 * - Skip all images
 * - Custom heading output
 * - Preserve specific HTML elements verbatim
 *
 * Compile:
 *   make visitor_pattern
 *   # or:
 *   cc -o visitor_pattern visitor_pattern.c $(pkg-config --cflags --libs html-to-markdown)
 */

#include <html_to_markdown.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Skip all images during conversion */
static HtmlToMarkdownVisitResult visit_image(
    const char *src, const char *alt, const char *title, void *user_data) {
    (void)src;
    (void)alt;
    (void)title;
    (void)user_data;
    printf("  [visitor] Skipping image: src=%s\n", src ? src : "(null)");
    return html_to_markdown_visit_result_skip();
}

/* Custom heading: wrap in brackets */
static HtmlToMarkdownVisitResult visit_heading(
    uint8_t level, const char *content, const char *id, void *user_data) {
    (void)id;
    (void)user_data;
    size_t len = strlen(content) + 32;
    char *output = (char *)malloc(len);
    snprintf(output, len, "[H%d] %s\n", level, content);
    printf("  [visitor] Custom heading level %d\n", level);
    return html_to_markdown_visit_result_custom(output);
}

/* Preserve <details> elements as raw HTML */
static HtmlToMarkdownVisitResult visit_details(
    const char *summary, const char *content, void *user_data) {
    (void)summary;
    (void)content;
    (void)user_data;
    printf("  [visitor] Preserving <details> as HTML\n");
    return html_to_markdown_visit_result_preserve_html();
}

int main(void) {
    printf("html-to-markdown visitor pattern example\n\n");

    /* Set up callbacks -- only the fields we care about */
    HtmlToMarkdownVisitorCallbacks callbacks = {0};
    callbacks.visit_image = (void *)visit_image;
    callbacks.visit_heading = (void *)visit_heading;
    callbacks.visit_details = (void *)visit_details;

    HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
    if (!visitor) {
        fprintf(stderr, "Failed to create visitor: %s\n", html_to_markdown_last_error());
        return 1;
    }

    const char *html =
        "<h1>Welcome</h1>"
        "<p>Some text here.</p>"
        "<img src=\"photo.jpg\" alt=\"A photo\" />"
        "<h2>Section</h2>"
        "<details><summary>Click me</summary><p>Hidden content</p></details>"
        "<p>Final paragraph.</p>";

    size_t len = 0;
    char *md = html_to_markdown_convert_with_visitor(html, visitor, &len);
    if (md) {
        printf("\nMarkdown output (%zu bytes):\n%s\n", len, md);
        html_to_markdown_free_string(md);
    } else {
        fprintf(stderr, "Conversion failed: %s\n", html_to_markdown_last_error());
    }

    html_to_markdown_visitor_free(visitor);
    return 0;
}
