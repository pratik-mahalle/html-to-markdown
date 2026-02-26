```r
library(htmltomarkdown)

html <- '
<html>
  <head><title>Example</title></head>
  <body>
    <h1 id="welcome">Welcome</h1>
    <a href="https://example.com">Example link</a>
  </body>
</html>'

result <- convert_with_metadata(html)

cat(result$markdown)
result$metadata$document$title
result$metadata$headers[[1]]$text
result$metadata$links[[1]]$link_type
```
