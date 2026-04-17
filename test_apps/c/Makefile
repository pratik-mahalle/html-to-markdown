# C FFI test app for html-to-markdown
# Downloads prebuilt library from GitHub releases.
# Usage: ./download_ffi.sh 3.2.4 && make test

CC = gcc
FFI_DIR = ffi

# Use downloaded FFI if available, fallback to pkg-config
ifneq ($(wildcard $(FFI_DIR)/include/html_to_markdown.h),)
    CFLAGS = -Wall -Wextra -I$(FFI_DIR)/include
    LDFLAGS = -L$(FFI_DIR)/lib -lhtml_to_markdown_ffi -Wl,-rpath,$(FFI_DIR)/lib
else ifneq ($(wildcard ../../crates/html-to-markdown-ffi/include/html_to_markdown.h),)
    CFLAGS = -Wall -Wextra -I../../crates/html-to-markdown-ffi/include
    LDFLAGS = -L../../target/release -lhtml_to_markdown_ffi -Wl,-rpath,../../target/release
else
    CFLAGS = -Wall -Wextra $(shell pkg-config --cflags html-to-markdown-ffi 2>/dev/null)
    LDFLAGS = $(shell pkg-config --libs html-to-markdown-ffi 2>/dev/null)
endif

SRCS = main.c test_conversion.c test_smoke.c
TARGET = run_tests

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

test: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)
