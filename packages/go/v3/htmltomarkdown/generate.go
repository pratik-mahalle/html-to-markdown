//go:generate go run github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/cmd/install@latest

// Package htmltomarkdown provides Go bindings for the html-to-markdown Rust library.
//
// The go:generate directive above downloads the FFI library for your platform
// and generates the CGO flags needed to build. Run it once after installing:
//
//	go generate github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown
//
// This eliminates the need to manually set CGO_CFLAGS and CGO_LDFLAGS environment variables.
//
// Alternatively, the package supports automatic runtime library download (see ffi_loader.go),
// but the static linking approach via go:generate is recommended for production builds
// as it produces fully self-contained binaries.
package htmltomarkdown
