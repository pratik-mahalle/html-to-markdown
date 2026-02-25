#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test error code names */
    assert(strcmp(html_to_markdown_error_code_name(0), "ok") == 0);
    assert(strcmp(html_to_markdown_error_code_name(1), "invalid_utf8") == 0);
    assert(strcmp(html_to_markdown_error_code_name(2), "parse") == 0);
    assert(strcmp(html_to_markdown_error_code_name(3), "visitor") == 0);
    assert(strcmp(html_to_markdown_error_code_name(4), "memory") == 0);
    assert(strcmp(html_to_markdown_error_code_name(5), "internal") == 0);
    assert(strcmp(html_to_markdown_error_code_name(99), "unknown") == 0);

    /* Test that NULL input triggers an error and sets error state */
    {
        const char *result = html_to_markdown_convert(NULL);
        assert(result == NULL);

        /* last_error should return a non-NULL, non-empty message */
        const char *err = html_to_markdown_last_error();
        assert(err != NULL);
        assert(strlen(err) > 0);

        /* last_error_code should return a non-zero code after an error */
        uint32_t code = html_to_markdown_last_error_code();
        assert(code != 0);

        /* error_code_name for the returned code should be a valid string */
        const char *name = html_to_markdown_error_code_name(code);
        assert(name != NULL);
        assert(strlen(name) > 0);
        assert(strcmp(name, "unknown") != 0);
    }

    /* Test that a successful conversion clears the error state */
    {
        char *result = html_to_markdown_convert("<p>hello</p>");
        assert(result != NULL);
        html_to_markdown_free_string(result);

        /* After a successful call, error code should be 0 (ok) */
        uint32_t code = html_to_markdown_last_error_code();
        assert(code == 0);
    }

    /* Test that free_string with NULL is a safe no-op */
    html_to_markdown_free_string(NULL);

    printf("test_error: all tests passed\n");
    return 0;
}
