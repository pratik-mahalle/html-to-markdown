/*
 * Minimal FFI declarations for C tests.
 *
 * The full html_to_markdown.h header has cbindgen-generated visitor types
 * that use incomplete struct fields. We extract only the declarations needed
 * for testing here.
 *
 * Version defines (HTML_TO_MARKDOWN_VERSION_MAJOR, etc.) are passed via
 * compiler -D flags from the Makefile, which extracts them from the
 * generated header to keep them in sync.
 */
#ifndef TEST_FFI_DECLS_H
#define TEST_FFI_DECLS_H

#include <stdbool.h>
#include <stdint.h>

/* Core conversion functions */
extern const char *html_to_markdown_version(void);
extern char *html_to_markdown_convert(const char *html, const char *options_json);
extern void html_to_markdown_free_string(char *s);

/* Error handling */
extern const char *html_to_markdown_last_error(void);
extern uint32_t html_to_markdown_last_error_code(void);
extern const char *html_to_markdown_error_code_name(uint32_t code);

/* Metadata conversion */
extern char *html_to_markdown_convert_with_metadata(const char *html, char **metadata_json_out);
extern char *html_to_markdown_convert_with_metadata_with_len(const char *html,
                                                             char **metadata_json_out,
                                                             uintptr_t *markdown_len_out,
                                                             uintptr_t *metadata_len_out);
extern char *html_to_markdown_convert_with_metadata_bytes_with_len(const uint8_t *html,
                                                                   uintptr_t len,
                                                                   char **metadata_json_out,
                                                                   uintptr_t *markdown_len_out,
                                                                   uintptr_t *metadata_len_out);

/* ------------------------------------------------------------------ */
/* Forward declarations for visitor types                             */
/* ------------------------------------------------------------------ */

typedef enum {
    HtmlToMarkdownVisitResultType_Continue = 0,
    HtmlToMarkdownVisitResultType_Custom = 1,
    HtmlToMarkdownVisitResultType_Skip = 2,
    HtmlToMarkdownVisitResultType_PreserveHtml = 3,
    HtmlToMarkdownVisitResultType_Error = 4,
} HtmlToMarkdownVisitResultType;

typedef struct {
    HtmlToMarkdownVisitResultType result_type;
    char *custom_output;
    char *error_message;
} HtmlToMarkdownVisitResult;

typedef void *HtmlToMarkdownVisitor;

/*
 * The callbacks struct has ~40 fields of opaque Option_*Callback types.
 * We use a raw byte buffer so we can zero-init it (all NULL callbacks = default).
 */
typedef struct {
    void *user_data;
    char _opaque_callbacks[sizeof(void *) * 80]; /* Oversized to accommodate all callback fields */
} HtmlToMarkdownVisitorCallbacksCompat;

/* ------------------------------------------------------------------ */
/* Visitor API                                                        */
/* ------------------------------------------------------------------ */

extern HtmlToMarkdownVisitor html_to_markdown_visitor_create(const void *callbacks);
extern void html_to_markdown_visitor_free(HtmlToMarkdownVisitor visitor);
extern char *html_to_markdown_convert_with_visitor(const char *html, HtmlToMarkdownVisitor visitor,
                                                   uintptr_t *len_out);
extern char *html_to_markdown_convert_bytes_with_visitor(const uint8_t *html, uintptr_t len,
                                                         HtmlToMarkdownVisitor visitor,
                                                         uintptr_t *len_out);

/* ------------------------------------------------------------------ */
/* Visit result constructors                                          */
/* ------------------------------------------------------------------ */

extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_continue(void);
extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_custom(char *output);
extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_skip(void);
extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_preserve_html(void);
extern HtmlToMarkdownVisitResult html_to_markdown_visit_result_error(char *message);

#endif /* TEST_FFI_DECLS_H */
