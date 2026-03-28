```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
)

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"

    result, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatal(err)
    }

    if result.Content != nil {
        fmt.Println(*result.Content)
    }
}
```
