```go
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
	html := `<html><head><title>My Page</title></head>
	<body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Markdown:", result.Markdown)
	fmt.Println("Title:", result.Metadata.Title)
	fmt.Println("Links:", result.Metadata.Links)
}
```
