#!/usr/bin/env Rscript
# E2E smoke test for the htmltomarkdown R package

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

# Options
opts <- conversion_options(heading_style = "underlined")
result <- convert_with_options("<h1>Test</h1>", opts)
stopifnot(grepl("====", result))
cat("PASS: conversion with options\n")

# Options handle
handle <- create_options_handle(opts)
result <- convert_with_options_handle("<h1>Handle</h1>", handle)
stopifnot(grepl("====", result))
cat("PASS: options handle\n")

# Metadata
meta <- convert_with_metadata("<html><head><title>Page</title></head><body><h1>H</h1></body></html>")
stopifnot(meta$metadata$document$title == "Page")
cat("PASS: metadata extraction\n")

# Inline images
img_result <- convert_with_inline_images("<p>No images</p>")
stopifnot(is.list(img_result))
stopifnot("markdown" %in% names(img_result))
cat("PASS: inline images\n")

cat("\nAll smoke tests passed!\n")
