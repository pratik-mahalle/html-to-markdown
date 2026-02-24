# Basic Usage - R

Basic HTML to Markdown conversion using htmltomarkdown.

## Simple Conversion

Convert HTML to Markdown with default options:

```r
library(htmltomarkdown)

markdown <- convert("<h1>Hello</h1>")
cat(markdown)
#> # Hello
```

## Error Handling

Conversion returns a character string or raises an error:

```r
tryCatch(
  convert("<h1>Hello</h1>"),
  error = function(e) message("Conversion failed: ", e$message)
)
```
