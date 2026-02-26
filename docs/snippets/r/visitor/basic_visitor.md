```r
library(htmltomarkdown)

html <- "<p>Visit <a href='https://example.com'>our site</a> for more!</p>"

markdown <- convert_with_visitor(html)
cat(markdown)
```
