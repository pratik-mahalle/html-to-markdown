```r
library(htmltomarkdown)

html <- "
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"

result <- convert_with_tables(html)

for (table in result$tables) {
  for (i in seq_along(table$cells)) {
    prefix <- if (table$is_header_row[[i]]) "Header" else "Row"
    cat(sprintf("  %s: %s\n", prefix, paste(table$cells[[i]], collapse = ", ")))
  }
}
```
