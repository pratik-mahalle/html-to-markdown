```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"

    markdown, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println(markdown)
}
```
