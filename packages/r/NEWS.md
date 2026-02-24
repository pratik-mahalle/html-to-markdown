# htmltomarkdown 2.25.1

* Initial CRAN release.
* High-performance HTML to Markdown conversion powered by a Rust backend via extendr.
* `convert()` for simple HTML-to-Markdown conversion.
* `convert_with_options()` and `convert_with_options_handle()` for customised conversion.
* `convert_with_metadata()` returns both Markdown output and document metadata.
* `convert_with_inline_images()` embeds images as base64 data URIs.
* `convert_with_visitor()` enables custom node-level transformation callbacks.
* `conversion_options()` helper for building option lists.
