```r
library(htmltomarkdown)

html <- "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
markdown <- convert(html)
cat(markdown)
```
