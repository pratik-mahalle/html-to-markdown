```r
library(htmltomarkdown)

opts <- conversion_options(
  heading_style = "atx",
  wrap = TRUE,
  wrap_width = 80L
)

markdown <- convert_with_options("<h1>Hello</h1><p>World</p>", opts)
cat(markdown)
```
