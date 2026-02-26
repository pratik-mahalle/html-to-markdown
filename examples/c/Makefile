# C examples for html-to-markdown-ffi
#
# Prerequisites:
#   - html-to-markdown-ffi installed (pkg-config --libs html-to-markdown works)
#   OR
#   - Set LIBDIR and HEADER_DIR to point at your build artifacts:
#       make LIBDIR=../../target/release HEADER_DIR=../../crates/html-to-markdown-ffi

HEADER_DIR ?= ../../crates/html-to-markdown-ffi
LIBDIR ?= ../../target/release
LIBNAME = html_to_markdown_ffi

CC ?= cc
CFLAGS = -Wall -Wextra -I$(HEADER_DIR)

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

EXAMPLES = basic_convert visitor_pattern metadata_extraction

.PHONY: all clean test

all: $(EXAMPLES)

basic_convert: basic_convert.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS) $(RPATH)

visitor_pattern: visitor_pattern.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS) $(RPATH)

metadata_extraction: metadata_extraction.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS) $(RPATH)

test: all
	@for e in $(EXAMPLES); do echo "Running $$e..."; ./$$e || exit 1; done

clean:
	rm -f $(EXAMPLES)
