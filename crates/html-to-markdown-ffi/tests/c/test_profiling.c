#include "test_ffi_decls.h"
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

int main(void) {
    /* Test profile_stop without start - should return false */
    {
        bool stopped = html_to_markdown_profile_stop();
        assert(!stopped);
    }

    /* Test profile_start with NULL path - should return false */
    {
        bool started = html_to_markdown_profile_start(NULL, 100);
        assert(!started);
    }

    /* Test profile_start with valid path and frequency */
    /* Note: profiling may not be available on all platforms,
       so we just check the return type is correct */
    {
        bool started = html_to_markdown_profile_start("/tmp/test_profile.svg", 100);
        if (started) {
            /* Do some work to profile */
            char *md = html_to_markdown_convert("<h1>Profile test</h1>");
            if (md)
                html_to_markdown_free_string(md);

            bool stopped = html_to_markdown_profile_stop();
            assert(stopped);
        }
        /* If start failed, that's OK - profiling may not be available */
    }

    printf("test_profiling: all tests passed\n");
    return 0;
}
