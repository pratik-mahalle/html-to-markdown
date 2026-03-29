# C test suite for html-to-markdown-ffi
# Usage: make [LIBDIR=path]

HEADER_DIR ?= ../..
HEADER ?= $(HEADER_DIR)/html_to_markdown.h
LIBDIR ?= ../../../../target/debug
LIBNAME = html_to_markdown_ffi

CC ?= cc
CFLAGS = -Wall -Wextra -Werror

# Extract version defines from generated header and pass as compiler flags
VERSION_MAJOR := $(shell grep 'define HTML_TO_MARKDOWN_VERSION_MAJOR' $(HEADER) | awk '{print $$3}')
VERSION_MINOR := $(shell grep 'define HTML_TO_MARKDOWN_VERSION_MINOR' $(HEADER) | awk '{print $$3}')
VERSION_PATCH := $(shell grep 'define HTML_TO_MARKDOWN_VERSION_PATCH' $(HEADER) | awk '{print $$3}')
VERSION_STR := $(shell grep 'define HTML_TO_MARKDOWN_VERSION ' $(HEADER) | sed 's/.*"\(.*\)"/\1/')

VERSION_DEFS = -DHTML_TO_MARKDOWN_VERSION_MAJOR=$(VERSION_MAJOR) \
               -DHTML_TO_MARKDOWN_VERSION_MINOR=$(VERSION_MINOR) \
               -DHTML_TO_MARKDOWN_VERSION_PATCH=$(VERSION_PATCH) \
               -DHTML_TO_MARKDOWN_VERSION=\"$(VERSION_STR)\"

ifeq ($(shell uname),Darwin)
  LDFLAGS = -L$(LIBDIR) -l$(LIBNAME) -framework CoreFoundation -framework Security -lpthread
  RPATH = -Wl,-rpath,$(LIBDIR)
else ifeq ($(OS),Windows_NT)
  LDFLAGS = -L$(LIBDIR) -l$(LIBNAME) -lws2_32 -luserenv -lbcrypt
  RPATH =
else
  LDFLAGS = -L$(LIBDIR) -l$(LIBNAME) -lpthread -ldl -lm
  RPATH = -Wl,-rpath,$(LIBDIR)
endif

TESTS = test_version test_error test_convert test_convert_extended test_visitor

.PHONY: all test clean

all: $(TESTS)

test_version: test_version.c test_ffi_decls.h
	$(CC) $(CFLAGS) $(VERSION_DEFS) -o $@ $< $(LDFLAGS) $(RPATH)

test_error: test_error.c test_ffi_decls.h
	$(CC) $(CFLAGS) $(VERSION_DEFS) -o $@ $< $(LDFLAGS) $(RPATH)

test_convert: test_convert.c test_ffi_decls.h
	$(CC) $(CFLAGS) $(VERSION_DEFS) -o $@ $< $(LDFLAGS) $(RPATH)

test_convert_extended: test_convert_extended.c test_ffi_decls.h
	$(CC) $(CFLAGS) $(VERSION_DEFS) -o $@ $< $(LDFLAGS) $(RPATH)

test_visitor: test_visitor.c test_ffi_decls.h
	$(CC) $(CFLAGS) $(VERSION_DEFS) -o $@ $< $(LDFLAGS) $(RPATH)

test: all
	@for t in $(TESTS); do echo "Running $$t..."; ./$$t || exit 1; done

clean:
	rm -f $(TESTS)
