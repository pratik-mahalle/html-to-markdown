# Metadata Extraction - R

Extract structured metadata from HTML documents during conversion.

## Basic Metadata Extraction

Use `convert_with_metadata()` to extract document metadata alongside Markdown:

```r
library(htmltomarkdown)

html <- '
<html>
  <head>
    <title>Example</title>
    <meta name="description" content="Demo page">
  </head>
  <body>
    <h1 id="welcome">Welcome</h1>
    <a href="https://example.com" rel="nofollow">Example link</a>
  </body>
</html>'

result <- convert_with_metadata(html)

cat(result$markdown)
result$metadata$document$title       # "Example"
result$metadata$headers[[1]]$text    # "Welcome"
result$metadata$links[[1]]$link_type # "external"
```

## Metadata Configuration

Control which metadata categories to extract:

```r
config <- list(
  extract_document = TRUE,
  extract_headers = TRUE,
  extract_links = TRUE,
  extract_images = FALSE,
  extract_structured_data = FALSE
)

result <- convert_with_metadata(html, options = NULL, config = config)
```

## Inline Image Extraction

Extract embedded images from HTML:

```r
html <- '<p><img src="data:image/png;base64,iVBOR..." alt="Logo"></p>'

config <- list(infer_dimensions = TRUE)
result <- convert_with_inline_images(html, options = NULL, config = config)

cat(result$markdown)
length(result$images)    # number of extracted images
length(result$warnings)  # any extraction warnings
```
