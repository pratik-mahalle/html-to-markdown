module github.com/kreuzberg-dev/html-to-markdown-test-app

go 1.25

require github.com/kreuzberg-dev/html-to-markdown/packages/go/v2 v2.25.0

// NOTE: The replace directive below is for development/testing within the monorepo.
// For production validation of published packages, this directive should be removed.
// The Go module will be published separately and available via:
// go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@v2.24.1
replace github.com/kreuzberg-dev/html-to-markdown/packages/go/v2 => ../../../packages/go/v2
