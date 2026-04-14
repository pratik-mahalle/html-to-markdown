#!/usr/bin/env Rscript
# E2E smoke test for the htmltomarkdown R package (v3 API)

library(htmltomarkdown)

# Basic conversion
result <- convert("<h1>Hello World</h1><p>Test paragraph</p>")
stopifnot(grepl("# Hello World", result))
stopifnot(grepl("Test paragraph", result))
cat("PASS: basic conversion\n")

# Version
v <- version()
stopifnot(nchar(v) > 0)
stopifnot(grepl("^\\d+\\.\\d+\\.\\d+$", v))
cat("PASS: version =", v, "\n")

# Empty input
result <- convert("")
stopifnot(result == "")
cat("PASS: empty input\n")

# Bold and italic
result <- convert("<p><strong>bold</strong> and <em>italic</em></p>")
stopifnot(grepl("bold", result))
stopifnot(grepl("italic", result))
cat("PASS: bold and italic conversion\n")

# Links
result <- convert('<a href="https://example.com">Link</a>')
stopifnot(grepl("Link", result))
stopifnot(grepl("https://example.com", result))
cat("PASS: link conversion\n")

# Lists
result <- convert("<ul><li>Item 1</li><li>Item 2</li></ul>")
stopifnot(grepl("Item 1", result))
stopifnot(grepl("Item 2", result))
cat("PASS: list conversion\n")

# Code block
result <- convert("<pre><code>code here</code></pre>")
stopifnot(grepl("code here", result))
cat("PASS: code block conversion\n")

cat("\nAll smoke tests passed!\n")
