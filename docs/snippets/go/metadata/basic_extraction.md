```go
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
)

func main() {
	html := `<html><head><title>My Page</title></head>
	<body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>`

	opts := htmltomarkdown.ConversionOptions{ExtractMetadata: true}
	result, err := htmltomarkdown.Convert(html, opts)
	if err != nil {
		log.Fatal(err)
	}

	if result.Content != nil {
		fmt.Println("Markdown:", *result.Content)
	}
	if result.Metadata != nil {
		fmt.Println("Title:", result.Metadata.Title)
		fmt.Println("Links:", result.Metadata.Links)
	}
}
```
