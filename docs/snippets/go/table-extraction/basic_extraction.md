```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"

html := `
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
`

opts := htmltomarkdown.ConversionOptions{ExtractTables: true}
result, err := htmltomarkdown.Convert(html, opts)
if err != nil {
    log.Fatal(err)
}

for _, table := range result.Tables {
    for i, row := range table.Cells {
        prefix := "Row"
        if table.IsHeaderRow[i] {
            prefix = "Header"
        }
        fmt.Printf("  %s: %v\n", prefix, row)
    }
}
```
