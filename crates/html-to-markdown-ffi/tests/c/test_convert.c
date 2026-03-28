#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test basic HTML to markdown conversion */
    const char *html = "<h1>Hello</h1><p>World</p>";
    char *result = html_to_markdown_convert(html, NULL);
    assert(result != NULL);
    assert(strlen(result) > 0);
    /* Should contain "Hello" and "World" */
    assert(strstr(result, "Hello") != NULL);
    assert(strstr(result, "World") != NULL);
    html_to_markdown_free_string(result);

    /* Test empty input */
    result = html_to_markdown_convert("", NULL);
    assert(result != NULL);
    html_to_markdown_free_string(result);

    /* Test NULL input returns NULL and sets error */
    result = html_to_markdown_convert(NULL, NULL);
    assert(result == NULL);

    /* Test free_string with NULL is safe */
    html_to_markdown_free_string(NULL);

    printf("test_convert: all tests passed\n");
    return 0;
}
