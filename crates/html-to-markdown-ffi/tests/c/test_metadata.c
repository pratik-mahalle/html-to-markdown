#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    /* Test convert_with_metadata - basic */
    {
        const char *html = "<html><head><title>Test Page</title></head><body><h1>Hello</h1></body></html>";
        char *metadata_json = NULL;
        char *markdown = html_to_markdown_convert_with_metadata(html, &metadata_json);
        assert(markdown != NULL);
        assert(strstr(markdown, "Hello") != NULL);
        /* metadata_json should be set (may be "{}" or contain title info) */
        assert(metadata_json != NULL);
        assert(strlen(metadata_json) > 0);
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }

    /* Test convert_with_metadata - NULL input */
    {
        char *metadata_json = NULL;
        char *markdown = html_to_markdown_convert_with_metadata(NULL, &metadata_json);
        assert(markdown == NULL);
    }

    /* Test convert_with_metadata_with_len */
    {
        const char *html = "<html><head><title>Len Test</title></head><body><p>Content</p></body></html>";
        char *metadata_json = NULL;
        uintptr_t md_len = 0, meta_len = 0;
        char *markdown = html_to_markdown_convert_with_metadata_with_len(
            html, &metadata_json, &md_len, &meta_len);
        assert(markdown != NULL);
        assert(md_len > 0);
        assert(md_len == strlen(markdown));
        assert(metadata_json != NULL);
        assert(meta_len > 0);
        assert(meta_len == strlen(metadata_json));
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }

    /* Test convert_with_metadata_bytes_with_len */
    {
        const char *html = "<html><body><p>Bytes test</p></body></html>";
        uintptr_t html_len = strlen(html);
        char *metadata_json = NULL;
        uintptr_t md_len = 0, meta_len = 0;
        char *markdown = html_to_markdown_convert_with_metadata_bytes_with_len(
            (const uint8_t *)html, html_len, &metadata_json, &md_len, &meta_len);
        assert(markdown != NULL);
        assert(md_len > 0);
        assert(md_len == strlen(markdown));
        assert(metadata_json != NULL);
        assert(meta_len > 0);
        assert(meta_len == strlen(metadata_json));
        assert(strstr(markdown, "Bytes test") != NULL);
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }

    printf("test_metadata: all tests passed\n");
    return 0;
}
