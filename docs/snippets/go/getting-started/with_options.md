```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    // Check library version
    version := htmltomarkdown.Version()
    fmt.Printf("html-to-markdown version: %s\n", version)

    html := "<h1>Hello</h1><p>Welcome</p>"

    // Convert with error handling
    markdown, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatalf("Conversion failed: %v", err)
    }

    fmt.Println(markdown)

    // Alternative: Use MustConvert for panicking on error
    anotherMarkdown := htmltomarkdown.MustConvert("<p>Safe HTML</p>")
    fmt.Println(anotherMarkdown)
}
```
