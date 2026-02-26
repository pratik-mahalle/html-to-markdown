#include "test_ffi_decls.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    /* Test visit result constructors (these are pure value functions, no visitor needed) */
    {
        HtmlToMarkdownVisitResult r;

        r = html_to_markdown_visit_result_continue();
        assert(r.result_type == HtmlToMarkdownVisitResultType_Continue);

        r = html_to_markdown_visit_result_skip();
        assert(r.result_type == HtmlToMarkdownVisitResultType_Skip);

        r = html_to_markdown_visit_result_preserve_html();
        assert(r.result_type == HtmlToMarkdownVisitResultType_PreserveHtml);
    }

    /* Test visit_result_custom */
    {
        char *custom = (char *)malloc(32);
        assert(custom != NULL);
        snprintf(custom, 32, "custom output");
        HtmlToMarkdownVisitResult r = html_to_markdown_visit_result_custom(custom);
        assert(r.result_type == HtmlToMarkdownVisitResultType_Custom);
        assert(r.custom_output != NULL);
        /* Note: custom string ownership transfers to the result */
    }

    /* Test visit_result_error */
    {
        char *msg = (char *)malloc(32);
        assert(msg != NULL);
        snprintf(msg, 32, "test error");
        HtmlToMarkdownVisitResult r = html_to_markdown_visit_result_error(msg);
        assert(r.result_type == HtmlToMarkdownVisitResultType_Error);
        assert(r.error_message != NULL);
        /* Note: error message ownership transfers to the result */
    }

    /* Test visitor create with zero-initialized callbacks (all defaults) */
    {
        HtmlToMarkdownVisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
        assert(visitor != NULL);

        /* Test convert_with_visitor using default callbacks */
        uintptr_t len_out = 0;
        char *md = html_to_markdown_convert_with_visitor(
            "<h1>Visitor Test</h1><p>Content</p>", visitor, &len_out);
        assert(md != NULL);
        assert(len_out > 0);
        assert(strstr(md, "Visitor Test") != NULL);
        html_to_markdown_free_string(md);

        html_to_markdown_visitor_free(visitor);
    }

    /* Test visitor_free with NULL is safe */
    html_to_markdown_visitor_free(NULL);

    /* Test convert_bytes_with_visitor */
    {
        HtmlToMarkdownVisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
        assert(visitor != NULL);

        const char *html = "<p>Bytes visitor test</p>";
        uintptr_t len_out = 0;
        char *md = html_to_markdown_convert_bytes_with_visitor(
            (const uint8_t *)html, strlen(html), visitor, &len_out);
        assert(md != NULL);
        assert(len_out > 0);
        assert(strstr(md, "Bytes visitor test") != NULL);
        html_to_markdown_free_string(md);

        html_to_markdown_visitor_free(visitor);
    }

    /* Test convert_with_visitor with NULL len_out (should be optional) */
    {
        HtmlToMarkdownVisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
        assert(visitor != NULL);

        char *md = html_to_markdown_convert_with_visitor(
            "<p>No len</p>", visitor, NULL);
        assert(md != NULL);
        assert(strstr(md, "No len") != NULL);
        html_to_markdown_free_string(md);

        html_to_markdown_visitor_free(visitor);
    }

    printf("test_visitor: all tests passed\n");
    return 0;
}
