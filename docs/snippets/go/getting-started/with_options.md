```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
)

func main() {
    // Check library version
    version := htmltomarkdown.Version()
    fmt.Printf("html-to-markdown version: %s\n", version)

    html := "<h1>Hello</h1><p>Welcome</p>"

    // Convert with error handling
    result, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatalf("Conversion failed: %v", err)
    }

    if result.Content != nil {
        fmt.Println(*result.Content)
    }
}
```
