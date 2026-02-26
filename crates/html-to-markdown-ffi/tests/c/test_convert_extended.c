#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test convert_with_len with valid HTML */
    {
        uintptr_t len = 0;
        char *result = html_to_markdown_convert_with_len("<h1>Hello</h1>", &len);
        assert(result != NULL);
        assert(len > 0);
        assert(len == strlen(result));
        assert(strstr(result, "Hello") != NULL);
        html_to_markdown_free_string(result);
    }

    /* Test convert_with_len with empty input */
    {
        uintptr_t len = 0;
        char *result = html_to_markdown_convert_with_len("", &len);
        assert(result != NULL);
        assert(len == strlen(result));
        html_to_markdown_free_string(result);
    }

    /* Test convert_with_len with NULL input */
    {
        uintptr_t len = 0;
        char *result = html_to_markdown_convert_with_len(NULL, &len);
        assert(result == NULL);
    }

    /* Test convert_bytes_with_len with valid UTF-8 bytes */
    {
        const char *html = "<p>World</p>";
        uintptr_t len_out = 0;
        char *result = html_to_markdown_convert_bytes_with_len(
            (const uint8_t *)html, strlen(html), &len_out);
        assert(result != NULL);
        assert(len_out > 0);
        assert(len_out == strlen(result));
        assert(strstr(result, "World") != NULL);
        html_to_markdown_free_string(result);
    }

    /* Test convert_bytes_with_len with zero length */
    {
        uintptr_t len_out = 0;
        char *result = html_to_markdown_convert_bytes_with_len(
            (const uint8_t *)"ignored", 0, &len_out);
        assert(result != NULL);
        assert(len_out == strlen(result));
        html_to_markdown_free_string(result);
    }

    /* Test convert_bytes_with_len with NULL pointer */
    {
        uintptr_t len_out = 0;
        char *result = html_to_markdown_convert_bytes_with_len(NULL, 0, &len_out);
        /* Should handle NULL gracefully */
        if (result != NULL) {
            html_to_markdown_free_string(result);
        }
    }

    printf("test_convert_extended: all tests passed\n");
    return 0;
}
