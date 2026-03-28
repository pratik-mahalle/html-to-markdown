#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test convert with valid HTML returns JSON containing content */
    {
        char *result = html_to_markdown_convert("<h1>Hello</h1>", NULL);
        assert(result != NULL);
        assert(strlen(result) > 0);
        /* Result is JSON; the content field should include "Hello" */
        assert(strstr(result, "Hello") != NULL);
        assert(strstr(result, "content") != NULL);
        html_to_markdown_free_string(result);
    }

    /* Test convert with empty input returns JSON */
    {
        char *result = html_to_markdown_convert("", NULL);
        assert(result != NULL);
        assert(strlen(result) > 0);
        /* Even empty input should produce valid JSON with content key */
        assert(strstr(result, "content") != NULL);
        html_to_markdown_free_string(result);
    }

    /* Test convert with NULL input returns NULL */
    {
        const char *result = html_to_markdown_convert(NULL, NULL);
        assert(result == NULL);
    }

    /* Test convert with paragraph HTML */
    {
        const char *html = "<p>World</p>";
        char *result = html_to_markdown_convert(html, NULL);
        assert(result != NULL);
        assert(strlen(result) > 0);
        assert(strstr(result, "World") != NULL);
        html_to_markdown_free_string(result);
    }

    /* Test convert with options_json set to "{}" (empty options object) */
    {
        char *result = html_to_markdown_convert("<p>Options test</p>", "{}");
        assert(result != NULL);
        assert(strstr(result, "Options test") != NULL);
        html_to_markdown_free_string(result);
    }

    printf("test_convert_extended: all tests passed\n");
    return 0;
}
