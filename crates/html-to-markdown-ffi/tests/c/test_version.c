#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test version string is non-NULL and non-empty */
    const char *version = html_to_markdown_version();
    assert(version != NULL);
    assert(strlen(version) > 0);

    /* Test version contains a dot (semver format) */
    assert(strchr(version, '.') != NULL);

    /* Test version defines match runtime version */
    char expected[64];
    snprintf(expected, sizeof(expected), "%d.%d.%d", HTML_TO_MARKDOWN_VERSION_MAJOR,
             HTML_TO_MARKDOWN_VERSION_MINOR, HTML_TO_MARKDOWN_VERSION_PATCH);
    assert(strcmp(version, expected) == 0);
    assert(strcmp(version, HTML_TO_MARKDOWN_VERSION) == 0);

    printf("test_version: all tests passed\n");
    return 0;
}
