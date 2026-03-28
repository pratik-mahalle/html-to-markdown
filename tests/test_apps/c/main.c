/*
 * html-to-markdown C FFI Test App
 *
 * Comprehensive test program for the html-to-markdown C FFI API.
 * Tests conversion, metadata extraction, error handling, visitor API,
 * version info, and profiling functions.
 *
 * Compile: see Makefile
 * Run:     ./htm_test
 */

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* -------------------------------------------------------------------------- */
/* FFI declarations                                                           */
/*                                                                            */
/* We declare extern functions directly rather than including the full         */
/* html_to_markdown.h header, which contains cbindgen-generated visitor types  */
/* with incomplete struct fields that C compilers reject. This approach keeps  */
/* the test self-contained and avoids header compatibility issues.             */
/* -------------------------------------------------------------------------- */

/* Core conversion — returns JSON string with "content", "document",          */
/* "metadata", "tables", and "warnings" fields.                               */
extern const char *html_to_markdown_version(void);
extern char *html_to_markdown_convert(const char *html, const char *options_json);
extern void html_to_markdown_free_string(char *s);

/* Error handling */
extern const char *html_to_markdown_last_error(void);
extern uint32_t html_to_markdown_last_error_code(void);
extern const char *html_to_markdown_error_code_name(uint32_t code);

/* Metadata conversion */
extern char *html_to_markdown_convert_with_metadata(const char *html,
                                                     char **metadata_json_out);
extern char *html_to_markdown_convert_with_metadata_with_len(
    const char *html, char **metadata_json_out, uintptr_t *markdown_len_out,
    uintptr_t *metadata_len_out);
extern char *html_to_markdown_convert_with_metadata_bytes_with_len(
    const uint8_t *html, uintptr_t len, char **metadata_json_out,
    uintptr_t *markdown_len_out, uintptr_t *metadata_len_out);

/* Visit result type */
typedef enum {
    VisitResultType_Continue = 0,
    VisitResultType_Custom = 1,
    VisitResultType_Skip = 2,
    VisitResultType_PreserveHtml = 3,
    VisitResultType_Error = 4,
} VisitResultType;

typedef struct {
    VisitResultType result_type;
    char *custom_output;
    char *error_message;
} HtmlToMarkdownVisitResult;

typedef void *HtmlToMarkdownVisitor;

/*
 * The callbacks struct has ~40 fields of opaque Option_*Callback types.
 * We zero-init a byte buffer (all NULL callbacks = default behavior).
 */
typedef struct {
    void *user_data;
    char _opaque_callbacks[sizeof(void *) * 80];
} VisitorCallbacksCompat;

/* Visitor API */
extern HtmlToMarkdownVisitor
html_to_markdown_visitor_create(const void *callbacks);
extern void html_to_markdown_visitor_free(HtmlToMarkdownVisitor visitor);
extern char *
html_to_markdown_convert_with_visitor(const char *html,
                                       HtmlToMarkdownVisitor visitor,
                                       uintptr_t *len_out);
extern char *html_to_markdown_convert_bytes_with_visitor(
    const uint8_t *html, uintptr_t len, HtmlToMarkdownVisitor visitor,
    uintptr_t *len_out);

/* Visit result constructors */
extern HtmlToMarkdownVisitResult
html_to_markdown_visit_result_continue(void);
extern HtmlToMarkdownVisitResult
html_to_markdown_visit_result_custom(char *output);
extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_skip(void);
extern HtmlToMarkdownVisitResult
html_to_markdown_visit_result_preserve_html(void);
extern HtmlToMarkdownVisitResult
html_to_markdown_visit_result_error(char *message);

/* Profiling */
                                            int32_t frequency);

/* -------------------------------------------------------------------------- */
/* Test runner                                                                */
/* -------------------------------------------------------------------------- */

typedef struct {
    int passed;
    int failed;
    int skipped;
    int section;
} TestRunner;

static TestRunner g_runner = {0, 0, 0, 0};

static void section(const char *name) {
    g_runner.section++;
    printf("\n[SECTION %d] %s\n", g_runner.section, name);
    printf("----------------------------------------------------------------------"
           "----------\n");
}

static void pass(const char *description) {
    printf("  PASS  %s\n", description);
    g_runner.passed++;
}

static void fail(const char *description, const char *detail) {
    printf("  FAIL  %s\n", description);
    if (detail) {
        printf("    Error: %s\n", detail);
    }
    g_runner.failed++;
}

static void skip(const char *description, const char *reason) {
    printf("  SKIP  %s (%s)\n", description, reason);
    g_runner.skipped++;
}

static int summary(void) {
    int total = g_runner.passed + g_runner.failed + g_runner.skipped;
    printf("\n======================================================================"
           "==========\n");
    printf("TEST SUMMARY\n");
    printf("======================================================================"
           "==========\n");
    printf("Total Tests: %d\n", total);
    printf("  Passed:  %d\n", g_runner.passed);
    printf("  Failed:  %d\n", g_runner.failed);
    printf("  Skipped: %d\n", g_runner.skipped);

    if (g_runner.failed == 0) {
        printf("\nALL TESTS PASSED\n");
        return 0;
    }
    printf("\n%d TEST(S) FAILED\n", g_runner.failed);
    return 1;
}

/* -------------------------------------------------------------------------- */
/* Section 1: Library Info                                                    */
/* -------------------------------------------------------------------------- */

static void test_library_info(void) {
    /* Test html_to_markdown_version */
    {
        const char *ver = html_to_markdown_version();
        if (ver && strlen(ver) > 0) {
            char buf[128];
            snprintf(buf, sizeof(buf),
                     "html_to_markdown_version() returns \"%s\"", ver);
            pass(buf);
        } else {
            fail("html_to_markdown_version() returns non-empty string",
                 "got NULL or empty");
        }
    }

    /* Test version contains a dot (semver) */
    {
        const char *ver = html_to_markdown_version();
        if (ver && strchr(ver, '.') != NULL) {
            pass("version string contains '.' (semver format)");
        } else {
            fail("version string contains '.'", "no dot found");
        }
    }

    /* Test html_to_markdown_last_error before any error */
    {
        /* Trigger a successful conversion first to clear state */
        char *r = html_to_markdown_convert("<p>init</p>", NULL);
        if (r)
            html_to_markdown_free_string(r);

        uint32_t code = html_to_markdown_last_error_code();
        if (code == 0) {
            pass("html_to_markdown_last_error_code() == 0 after success");
        } else {
            fail("html_to_markdown_last_error_code() == 0 after success",
                 "non-zero code");
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Section 2: Error Code Functions                                            */
/* -------------------------------------------------------------------------- */

static void test_error_codes(void) {
    /* Test all error code names */
    struct {
        uint32_t code;
        const char *expected;
    } codes[] = {
        {0, "ok"},      {1, "invalid_utf8"}, {2, "parse"},
        {3, "visitor"},  {4, "memory"},       {5, "internal"},
    };

    for (size_t i = 0; i < sizeof(codes) / sizeof(codes[0]); i++) {
        const char *name = html_to_markdown_error_code_name(codes[i].code);
        if (name && strcmp(name, codes[i].expected) == 0) {
            char buf[128];
            snprintf(buf, sizeof(buf),
                     "error_code_name(%u) == \"%s\"", codes[i].code,
                     codes[i].expected);
            pass(buf);
        } else {
            char buf[128];
            snprintf(buf, sizeof(buf), "expected \"%s\", got \"%s\"",
                     codes[i].expected, name ? name : "NULL");
            fail("error_code_name() mismatch", buf);
        }
    }

    /* Test invalid error code */
    {
        const char *name = html_to_markdown_error_code_name(99);
        if (name && strcmp(name, "unknown") == 0) {
            pass("error_code_name(99) == \"unknown\"");
        } else {
            fail("error_code_name(99) == \"unknown\"",
                 name ? name : "NULL");
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Section 3: Basic Conversion                                                */
/* -------------------------------------------------------------------------- */
/*                                                                            */
/* html_to_markdown_convert() returns a JSON string of the form:             */
/*   {"content":"...","document":...,"metadata":...,"tables":[...],...}      */
/* We check that the JSON result contains expected text fragments.            */
/* -------------------------------------------------------------------------- */

static void test_basic_conversion(void) {
    /* Simple HTML to Markdown: result is JSON containing the content */
    {
        const char *html = "<h1>Hello</h1><p>World</p>";
        char *result = html_to_markdown_convert(html, NULL);
        if (result && strstr(result, "Hello") && strstr(result, "World")) {
            pass("convert(<h1>Hello</h1><p>World</p>) JSON contains both words");
        } else {
            fail("convert() basic test", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Heading produces '#' somewhere in the JSON content string */
    {
        char *result = html_to_markdown_convert("<h1>Title</h1>", NULL);
        if (result && strstr(result, "#")) {
            pass("convert(<h1>Title</h1>) JSON contains '#'");
        } else {
            fail("heading conversion", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Paragraph text */
    {
        char *result = html_to_markdown_convert("<p>paragraph</p>", NULL);
        if (result && strstr(result, "paragraph")) {
            pass("convert(<p>paragraph</p>) JSON contains 'paragraph'");
        } else {
            fail("paragraph conversion", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Bold text */
    {
        char *result = html_to_markdown_convert("<strong>bold</strong>", NULL);
        if (result && strstr(result, "**bold**")) {
            pass("convert(<strong>bold</strong>) JSON contains '**bold**'");
        } else {
            fail("bold conversion",
                 result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Italic text */
    {
        char *result = html_to_markdown_convert("<em>italic</em>", NULL);
        if (result && strstr(result, "*italic*")) {
            pass("convert(<em>italic</em>) JSON contains '*italic*'");
        } else {
            fail("italic conversion",
                 result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Link */
    {
        char *result =
            html_to_markdown_convert("<a href=\"https://example.com\">link</a>", NULL);
        if (result && strstr(result, "https://example.com") &&
            strstr(result, "link")) {
            pass("convert(<a>) JSON contains URL and text");
        } else {
            fail("link conversion", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Empty input */
    {
        char *result = html_to_markdown_convert("", NULL);
        if (result != NULL) {
            pass("convert(\"\") returns non-NULL");
        } else {
            fail("convert(\"\") returns non-NULL", "got NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Complex nested HTML */
    {
        const char *html =
            "<div><h2>Sub</h2><ul><li>Item 1</li><li>Item 2</li></ul></div>";
        char *result = html_to_markdown_convert(html, NULL);
        if (result && strstr(result, "Sub") && strstr(result, "Item 1")) {
            pass("convert() JSON handles nested HTML (headings + lists)");
        } else {
            fail("nested HTML conversion", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }

    /* Unicode content */
    {
        char *result = html_to_markdown_convert(
            "<p>\xc3\xa9\xc3\xa0\xc3\xbc \xe4\xb8\xad\xe6\x96\x87</p>", NULL);
        if (result && strlen(result) > 0) {
            pass("convert() handles Unicode content");
        } else {
            fail("Unicode conversion", result ? result : "NULL");
        }
        html_to_markdown_free_string(result);
    }
}

/* -------------------------------------------------------------------------- */
/* Section 4: Metadata Conversion                                            */
/* -------------------------------------------------------------------------- */

static void test_metadata_conversion(void) {
    /* convert_with_metadata basic */
    {
        const char *html = "<html><head><title>Test Page</title></head>"
                           "<body><h1>Hello</h1></body></html>";
        char *metadata_json = NULL;
        char *markdown =
            html_to_markdown_convert_with_metadata(html, &metadata_json);
        if (markdown && strstr(markdown, "Hello")) {
            pass("convert_with_metadata() returns markdown with content");
        } else {
            fail("convert_with_metadata() markdown", markdown ? markdown : "NULL");
        }
        if (metadata_json && strlen(metadata_json) > 0) {
            pass("convert_with_metadata() returns non-empty metadata JSON");
        } else {
            fail("convert_with_metadata() metadata", "NULL or empty");
        }
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }

    /* convert_with_metadata NULL input */
    {
        char *metadata_json = NULL;
        const char *markdown =
            html_to_markdown_convert_with_metadata(NULL, &metadata_json);
        if (markdown == NULL) {
            pass("convert_with_metadata(NULL) returns NULL");
        } else {
            fail("convert_with_metadata(NULL)", "expected NULL");
        }
    }

    /* convert_with_metadata_with_len */
    {
        const char *html = "<html><head><title>Len</title></head>"
                           "<body><p>Content</p></body></html>";
        char *metadata_json = NULL;
        uintptr_t md_len = 0, meta_len = 0;
        char *markdown = html_to_markdown_convert_with_metadata_with_len(
            html, &metadata_json, &md_len, &meta_len);
        if (markdown && md_len > 0 && md_len == strlen(markdown)) {
            pass("convert_with_metadata_with_len() markdown length correct");
        } else {
            fail("convert_with_metadata_with_len() markdown", "length mismatch");
        }
        if (metadata_json && meta_len > 0 && meta_len == strlen(metadata_json)) {
            pass("convert_with_metadata_with_len() metadata length correct");
        } else {
            fail("convert_with_metadata_with_len() metadata", "length mismatch");
        }
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }

    /* convert_with_metadata_bytes_with_len */
    {
        const char *html = "<html><body><p>Bytes test</p></body></html>";
        uintptr_t html_len = strlen(html);
        char *metadata_json = NULL;
        uintptr_t md_len = 0, meta_len = 0;
        char *markdown =
            html_to_markdown_convert_with_metadata_bytes_with_len(
                (const uint8_t *)html, html_len, &metadata_json, &md_len,
                &meta_len);
        if (markdown && md_len > 0 && strstr(markdown, "Bytes test")) {
            pass("convert_with_metadata_bytes_with_len() works correctly");
        } else {
            fail("convert_with_metadata_bytes_with_len()",
                 markdown ? markdown : "NULL");
        }
        html_to_markdown_free_string(markdown);
        html_to_markdown_free_string(metadata_json);
    }
}

/* -------------------------------------------------------------------------- */
/* Section 5: Error Handling                                                  */
/* -------------------------------------------------------------------------- */

static void test_error_handling(void) {
    /* NULL input triggers error */
    {
        const char *result = html_to_markdown_convert(NULL, NULL);
        if (result == NULL) {
            pass("convert(NULL) returns NULL");
        } else {
            fail("convert(NULL)", "expected NULL");
        }
    }

    /* Error state is set after failure */
    {
        html_to_markdown_convert(NULL, NULL); /* trigger error */
        const char *err = html_to_markdown_last_error();
        if (err && strlen(err) > 0) {
            pass("last_error() returns non-empty message after failure");
        } else {
            fail("last_error() after failure", "NULL or empty");
        }

        uint32_t code = html_to_markdown_last_error_code();
        if (code != 0) {
            pass("last_error_code() != 0 after failure");
        } else {
            fail("last_error_code() != 0", "was 0");
        }

        const char *name = html_to_markdown_error_code_name(code);
        if (name && strcmp(name, "unknown") != 0) {
            char buf[128];
            snprintf(buf, sizeof(buf),
                     "error_code_name(%u) returns \"%s\" (known code)", code,
                     name);
            pass(buf);
        } else {
            fail("error_code_name() for returned code", "unknown");
        }
    }

    /* Successful conversion clears error state */
    {
        char *result = html_to_markdown_convert("<p>ok</p>", NULL);
        if (result)
            html_to_markdown_free_string(result);
        uint32_t code = html_to_markdown_last_error_code();
        if (code == 0) {
            pass("last_error_code() == 0 after successful conversion");
        } else {
            fail("error state cleared after success", "code != 0");
        }
    }

    /* free_string(NULL) is safe */
    {
        html_to_markdown_free_string(NULL);
        pass("free_string(NULL) is a safe no-op");
    }
}

/* -------------------------------------------------------------------------- */
/* Section 6: Visitor API                                                     */
/* -------------------------------------------------------------------------- */

static void test_visitor_api(void) {
    /* Visit result constructors */
    {
        HtmlToMarkdownVisitResult r =
            html_to_markdown_visit_result_continue();
        if (r.result_type == VisitResultType_Continue) {
            pass("visit_result_continue() has correct type");
        } else {
            fail("visit_result_continue()", "wrong type");
        }
    }

    {
        HtmlToMarkdownVisitResult r = html_to_markdown_visit_result_skip();
        if (r.result_type == VisitResultType_Skip) {
            pass("visit_result_skip() has correct type");
        } else {
            fail("visit_result_skip()", "wrong type");
        }
    }

    {
        HtmlToMarkdownVisitResult r =
            html_to_markdown_visit_result_preserve_html();
        if (r.result_type == VisitResultType_PreserveHtml) {
            pass("visit_result_preserve_html() has correct type");
        } else {
            fail("visit_result_preserve_html()", "wrong type");
        }
    }

    /* visit_result_custom */
    {
        char *custom = (char *)malloc(32);
        if (!custom) {
            skip("visit_result_custom()", "OOM");
        } else {
            snprintf(custom, 32, "custom output");
            HtmlToMarkdownVisitResult r =
                html_to_markdown_visit_result_custom(custom);
            if (r.result_type == VisitResultType_Custom &&
                r.custom_output != NULL) {
                pass("visit_result_custom() has correct type and output");
            } else {
                fail("visit_result_custom()", "wrong type or NULL output");
            }
            /* Ownership transferred; don't free custom */
        }
    }

    /* visit_result_error */
    {
        char *msg = (char *)malloc(32);
        if (!msg) {
            skip("visit_result_error()", "OOM");
        } else {
            snprintf(msg, 32, "test error");
            HtmlToMarkdownVisitResult r =
                html_to_markdown_visit_result_error(msg);
            if (r.result_type == VisitResultType_Error &&
                r.error_message != NULL) {
                pass("visit_result_error() has correct type and message");
            } else {
                fail("visit_result_error()", "wrong type or NULL message");
            }
            /* Ownership transferred; don't free msg */
        }
    }

    /* Visitor create with zero-initialized callbacks (all defaults) */
    {
        VisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor =
            html_to_markdown_visitor_create(&callbacks);
        if (visitor != NULL) {
            pass("visitor_create() with zero callbacks returns non-NULL");

            /* Convert with visitor */
            uintptr_t len_out = 0;
            char *md = html_to_markdown_convert_with_visitor(
                "<h1>Visitor Test</h1><p>Content</p>", visitor, &len_out);
            if (md && len_out > 0 && strstr(md, "Visitor Test")) {
                pass("convert_with_visitor() returns correct markdown");
            } else {
                fail("convert_with_visitor()", md ? md : "NULL");
            }
            html_to_markdown_free_string(md);

            html_to_markdown_visitor_free(visitor);
        } else {
            fail("visitor_create()", "returned NULL");
        }
    }

    /* visitor_free(NULL) is safe */
    {
        html_to_markdown_visitor_free(NULL);
        pass("visitor_free(NULL) is a safe no-op");
    }

    /* convert_bytes_with_visitor */
    {
        VisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor =
            html_to_markdown_visitor_create(&callbacks);
        if (visitor) {
            const char *html = "<p>Bytes visitor test</p>";
            uintptr_t len_out = 0;
            char *md = html_to_markdown_convert_bytes_with_visitor(
                (const uint8_t *)html, strlen(html), visitor, &len_out);
            if (md && len_out > 0 && strstr(md, "Bytes visitor test")) {
                pass("convert_bytes_with_visitor() works correctly");
            } else {
                fail("convert_bytes_with_visitor()", md ? md : "NULL");
            }
            html_to_markdown_free_string(md);
            html_to_markdown_visitor_free(visitor);
        } else {
            fail("visitor_create() for bytes test", "returned NULL");
        }
    }

    /* convert_with_visitor with NULL len_out */
    {
        VisitorCallbacksCompat callbacks;
        memset(&callbacks, 0, sizeof(callbacks));
        HtmlToMarkdownVisitor visitor =
            html_to_markdown_visitor_create(&callbacks);
        if (visitor) {
            char *md = html_to_markdown_convert_with_visitor(
                "<p>No len</p>", visitor, NULL);
            if (md && strstr(md, "No len")) {
                pass("convert_with_visitor(len_out=NULL) works");
            } else {
                fail("convert_with_visitor(len_out=NULL)",
                     md ? md : "NULL");
            }
            html_to_markdown_free_string(md);
            html_to_markdown_visitor_free(visitor);
        } else {
            fail("visitor_create() for NULL len_out test", "returned NULL");
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Section 7: Profiling API                                                   */
/* -------------------------------------------------------------------------- */

static void test_profiling_api(void) {
    {
        if (!stopped) {
        } else {
            fail("profile_stop() without start", "returned true");
        }
    }

    /* profile_start with NULL path should return false */
    {
        if (!started) {
            pass("profile_start(NULL) returns false");
        } else {
            fail("profile_start(NULL)", "returned true");
        }
    }

    /* profile_start + convert + profile_stop (platform-dependent) */
    {
        bool started =
        if (started) {
            char *md = html_to_markdown_convert("<h1>Profile test</h1>", NULL);
            if (md)
                html_to_markdown_free_string(md);
            if (stopped) {
                pass("profile start/stop cycle completed successfully");
            } else {
                fail("profile_stop() after start", "returned false");
            }
        } else {
            skip("profile start/stop cycle",
                 "profiling not available on this platform");
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Section 8: Memory Safety                                                   */
/* -------------------------------------------------------------------------- */

static void test_memory_safety(void) {
    /* Repeated conversions don't leak (stress test) */
    {
        int count = 100;
        int ok = 1;
        for (int i = 0; i < count; i++) {
            char *result =
                html_to_markdown_convert("<p>stress test iteration</p>", NULL);
            if (!result) {
                ok = 0;
                break;
            }
            html_to_markdown_free_string(result);
        }
        if (ok) {
            char buf[128];
            snprintf(buf, sizeof(buf), "%d repeated conversions succeed",
                     count);
            pass(buf);
        } else {
            fail("repeated conversions", "NULL returned");
        }
    }

    /* Alternating success/failure */
    {
        for (int i = 0; i < 10; i++) {
            html_to_markdown_convert(NULL, NULL); /* failure */
            char *result = html_to_markdown_convert("<p>ok</p>", NULL);
            if (result)
                html_to_markdown_free_string(result);
        }
        pass("alternating success/failure cycles complete without crash");
    }
}

/* -------------------------------------------------------------------------- */
/* main                                                                       */
/* -------------------------------------------------------------------------- */

int main(void) {
    printf("======================================================================"
           "==========\n");
    printf("HTML-TO-MARKDOWN C FFI COMPREHENSIVE TEST SUITE\n");
    printf("======================================================================"
           "==========\n");
    printf("Library version: %s\n", html_to_markdown_version());

    section("Library Info");
    test_library_info();

    section("Error Code Functions");
    test_error_codes();

    section("Basic Conversion");
    test_basic_conversion();

    section("Metadata Conversion");
    test_metadata_conversion();

    section("Error Handling");
    test_error_handling();

    section("Visitor API");
    test_visitor_api();

    section("Profiling API");
    test_profiling_api();

    section("Memory Safety");
    test_memory_safety();

    return summary();
}
