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

#include <stdint.h>

/* Core conversion functions */
extern const char *html_to_markdown_version(void);
extern char *html_to_markdown_convert(const char *html);
extern void html_to_markdown_free_string(char *s);

/* Error handling */
extern const char *html_to_markdown_last_error(void);
extern uint32_t html_to_markdown_last_error_code(void);
extern const char *html_to_markdown_error_code_name(uint32_t code);

#endif /* TEST_FFI_DECLS_H */
