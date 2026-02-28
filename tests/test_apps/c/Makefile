# html-to-markdown C FFI Test App - Makefile
#
# Builds and runs the comprehensive C FFI test suite.
#
# Usage:
#   make              Build the test binary
#   make test         Build and run tests
#   make clean        Remove build artifacts
#
# Configuration:
#   HTM_ROOT          Path to html-to-markdown repo root (default: ../../..)
#   CC                C compiler (default: cc)
#   BUILD_MODE        debug or release (default: release)

HTM_ROOT ?= ../../..
BUILD_MODE ?= release

HEADER_DIR = $(HTM_ROOT)/crates/html-to-markdown-ffi
HEADER = $(HEADER_DIR)/html_to_markdown.h
LIBDIR = $(HTM_ROOT)/target/$(BUILD_MODE)
LIBNAME = html_to_markdown_ffi

CC ?= cc
CFLAGS = -std=c11 -Wall -Wextra -Wpedantic

# Platform-specific linker flags
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

$(info Using LIBDIR=$(LIBDIR))

BINARY = htm_test

.PHONY: all build test clean help

all: build

build: $(BINARY)

$(BINARY): main.c
	$(CC) $(CFLAGS) $(RPATH) -o $@ main.c $(LDFLAGS)

test: build
	@echo "--- Running html-to-markdown C FFI Test Suite ---"
	./$(BINARY)

clean:
	rm -f $(BINARY)

help:
	@echo "html-to-markdown C FFI Test App"
	@echo ""
	@echo "Targets: build (default), test, clean, help"
	@echo "Variables: HTM_ROOT (default: ../../..), CC (default: cc), BUILD_MODE (default: release)"
	@echo ""
	@echo "Prerequisites:"
	@echo "  cargo build --release -p html-to-markdown-ffi"
	@echo ""
	@echo "Example:"
	@echo "  cargo build --release -p html-to-markdown-ffi && make test"
