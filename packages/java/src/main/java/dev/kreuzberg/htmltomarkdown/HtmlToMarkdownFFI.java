package dev.kreuzberg.htmltomarkdown;

import dev.kreuzberg.htmltomarkdown.util.StringUtils;
import java.io.IOException;
import java.io.InputStream;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

/**
 * Low-level Foreign Function Interface (FFI) bindings to the native html-to-markdown library.
 *
 * <p>This class provides direct access to the C FFI functions using Java's Foreign Function &amp;
 * Memory API (Panama). For a higher-level, more ergonomic API, use {@link HtmlToMarkdown} instead.
 *
 * @since 2.7.3
 */
final class HtmlToMarkdownFFI {

  /** Library name constant. */
  private static final String LIBRARY_NAME = "html_to_markdown_ffi";

  /** Native linker instance. */
  private static final Linker LINKER = Linker.nativeLinker();

  /** Symbol lookup for native functions. */
  private static final SymbolLookup SYMBOL_LOOKUP;

  /** Function descriptor for convert. */
  private static final FunctionDescriptor CONVERT_DESC =
      FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for free string. */
  private static final FunctionDescriptor FREE_STRING_DESC =
      FunctionDescriptor.ofVoid(ValueLayout.ADDRESS);

  /** Function descriptor for version. */
  private static final FunctionDescriptor VERSION_DESC = FunctionDescriptor.of(ValueLayout.ADDRESS);

  /** Function descriptor for last error. */
  private static final FunctionDescriptor LAST_ERROR_DESC =
      FunctionDescriptor.of(ValueLayout.ADDRESS);

  /** Function descriptor for convert with metadata. */
  private static final FunctionDescriptor CONVERT_WITH_METADATA_DESC =
      FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for profile start. */
  private static final FunctionDescriptor PROFILE_START_DESC =
      FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.JAVA_INT);

  /** Function descriptor for profile stop. */
  private static final FunctionDescriptor PROFILE_STOP_DESC =
      FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN);

  /** Method handle for html_to_markdown_convert. */
  static final MethodHandle html_to_markdown_convert;

  /** Method handle for html_to_markdown_free_string. */
  static final MethodHandle html_to_markdown_free_string;

  /** Method handle for html_to_markdown_version. */
  static final MethodHandle html_to_markdown_version;

  /** Method handle for html_to_markdown_last_error. */
  static final MethodHandle html_to_markdown_last_error;

  /** Method handle for html_to_markdown_convert_with_metadata. */
  static final MethodHandle html_to_markdown_convert_with_metadata;

  /** Method handle for html_to_markdown_profile_start. */
  static final MethodHandle html_to_markdown_profile_start;

  /** Method handle for html_to_markdown_profile_stop. */
  static final MethodHandle html_to_markdown_profile_stop;

  /** Method handle for html_to_markdown_convert_with_tables. */
  static final MethodHandle html_to_markdown_convert_with_tables;

  /** Method handle for html_to_markdown_extract. */
  static final MethodHandle html_to_markdown_extract;

  /** Method handle for html_to_markdown_visitor_create. */
  static final MethodHandle html_to_markdown_visitor_create;

  /** Method handle for html_to_markdown_visitor_free. */
  static final MethodHandle html_to_markdown_visitor_free;

  /** Method handle for html_to_markdown_convert_with_visitor. */
  static final MethodHandle html_to_markdown_convert_with_visitor;

  /** Function descriptor for convert with tables. */
  private static final FunctionDescriptor CONVERT_WITH_TABLES_DESC =
      FunctionDescriptor.of(
          ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for extract (html, options_json -> json string). */
  private static final FunctionDescriptor EXTRACT_DESC =
      FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for visitor create. */
  private static final FunctionDescriptor VISITOR_CREATE_DESC =
      FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for visitor free. */
  private static final FunctionDescriptor VISITOR_FREE_DESC =
      FunctionDescriptor.ofVoid(ValueLayout.ADDRESS);

  /** Function descriptor for convert with visitor. */
  private static final FunctionDescriptor CONVERT_WITH_VISITOR_DESC =
      FunctionDescriptor.of(
          ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /**
   * Load the native library either from java.library.path or by extracting from classpath.
   *
   * <p>This method first attempts to load the library using the standard System.loadLibrary(),
   * which respects the java.library.path system property. This is useful for development and for
   * users who prefer to manage native libraries manually.
   *
   * <p>If that fails, the method attempts to extract the appropriate platform-specific native
   * library from the classpath (where it is bundled as a resource) to a temporary directory and
   * load it from there. This allows the library to work out-of-the-box when installed from Maven
   * Central without requiring any manual native library management.
   *
   * @throws UnsatisfiedLinkError if the native library cannot be loaded
   */
  private static void loadNativeLibrary() {
    // Try java.library.path first (development / manual configuration)
    try {
      System.loadLibrary(LIBRARY_NAME);
      return;
    } catch (UnsatisfiedLinkError ignored) {
      // Fall through to classpath extraction
    }

    // Detect platform
    final String os = detectOs();
    final String arch = detectArch();
    final String platform = os + "-" + arch;
    final String libName = System.mapLibraryName(LIBRARY_NAME);

    // Look for bundled native library in classpath
    final String resourcePath = "/native/" + platform + "/" + libName;
    try (InputStream in = HtmlToMarkdownFFI.class.getResourceAsStream(resourcePath)) {
      if (in == null) {
        throw new UnsatisfiedLinkError(
            "Native library not found for platform "
                + platform
                + ". Resource path: "
                + resourcePath
                + "\n"
                + "The html-to-markdown Java package requires a native library for your platform.\n"
                + "This usually means the native library was not bundled in the JAR.\n"
                + "Supported platforms: linux-x86_64, linux-aarch64, osx-x86_64, osx-aarch64,"
                + " windows-x86_64");
      }

      // Extract to temp directory
      final Path tempDir = Files.createTempDirectory("html-to-markdown-native");
      final Path tempLib = tempDir.resolve(libName);
      Files.copy(in, tempLib, StandardCopyOption.REPLACE_EXISTING);

      // Mark for deletion on exit
      tempLib.toFile().deleteOnExit();
      tempDir.toFile().deleteOnExit();

      // Load from extracted path
      System.load(tempLib.toAbsolutePath().toString());

    } catch (IOException e) {
      throw new UnsatisfiedLinkError("Failed to extract native library: " + e.getMessage());
    }
  }

  /**
   * Detect the operating system.
   *
   * @return "linux", "osx", or "windows"
   * @throws UnsatisfiedLinkError if the OS is not supported
   */
  private static String detectOs() {
    final String osName = System.getProperty("os.name").toLowerCase();
    if (osName.contains("linux")) {
      return "linux";
    } else if (osName.contains("mac") || osName.contains("darwin")) {
      return "osx";
    } else if (osName.contains("windows")) {
      return "windows";
    } else {
      throw new UnsatisfiedLinkError("Unsupported operating system: " + osName);
    }
  }

  /**
   * Detect the CPU architecture.
   *
   * @return "x86_64" or "aarch64"
   * @throws UnsatisfiedLinkError if the architecture is not supported
   */
  private static String detectArch() {
    final String osArch = System.getProperty("os.arch").toLowerCase();
    if (osArch.contains("amd64") || osArch.contains("x86_64")) {
      return "x86_64";
    } else if (osArch.contains("aarch64") || osArch.contains("arm64")) {
      return "aarch64";
    } else {
      throw new UnsatisfiedLinkError("Unsupported architecture: " + osArch);
    }
  }

  static {
    loadNativeLibrary();

    SYMBOL_LOOKUP = SymbolLookup.loaderLookup();

    html_to_markdown_convert =
        LINKER.downcallHandle(findSymbol("html_to_markdown_convert"), CONVERT_DESC);

    html_to_markdown_free_string =
        LINKER.downcallHandle(findSymbol("html_to_markdown_free_string"), FREE_STRING_DESC);

    html_to_markdown_version =
        LINKER.downcallHandle(findSymbol("html_to_markdown_version"), VERSION_DESC);

    html_to_markdown_last_error =
        LINKER.downcallHandle(findSymbol("html_to_markdown_last_error"), LAST_ERROR_DESC);

    html_to_markdown_convert_with_metadata =
        LINKER.downcallHandle(
            findSymbol("html_to_markdown_convert_with_metadata"), CONVERT_WITH_METADATA_DESC);

    html_to_markdown_profile_start =
        LINKER.downcallHandle(findSymbol("html_to_markdown_profile_start"), PROFILE_START_DESC);

    html_to_markdown_profile_stop =
        LINKER.downcallHandle(findSymbol("html_to_markdown_profile_stop"), PROFILE_STOP_DESC);

    html_to_markdown_convert_with_tables =
        LINKER.downcallHandle(
            findSymbol("html_to_markdown_convert_with_tables"), CONVERT_WITH_TABLES_DESC);

    html_to_markdown_extract =
        LINKER.downcallHandle(findSymbol("html_to_markdown_extract"), EXTRACT_DESC);

    html_to_markdown_visitor_create =
        LINKER.downcallHandle(findSymbol("html_to_markdown_visitor_create"), VISITOR_CREATE_DESC);

    html_to_markdown_visitor_free =
        LINKER.downcallHandle(findSymbol("html_to_markdown_visitor_free"), VISITOR_FREE_DESC);

    html_to_markdown_convert_with_visitor =
        LINKER.downcallHandle(
            findSymbol("html_to_markdown_convert_with_visitor"), CONVERT_WITH_VISITOR_DESC);
  }

  /**
   * Find a native symbol in the loaded library.
   *
   * @param name the symbol name
   * @return the memory address of the symbol
   * @throws UnsatisfiedLinkError if the symbol cannot be found
   */
  private static MemorySegment findSymbol(final String name) {
    return SYMBOL_LOOKUP
        .find(name)
        .orElseThrow(() -> new UnsatisfiedLinkError("Symbol not found: " + name));
  }

  /**
   * Convert a Java String to a native C string (null-terminated).
   *
   * <p>The returned MemorySegment must be closed by the caller to free the native memory.
   *
   * @param arena the arena allocator to use
   * @param str the Java string to convert
   * @return a MemorySegment containing the null-terminated C string
   */
  static MemorySegment toCString(final Arena arena, final String str) {
    return StringUtils.toCString(arena, str);
  }

  /**
   * Convert a native C string to a Java String.
   *
   * @param addr the memory address of the C string
   * @return the Java string, or null if addr is NULL
   */
  static String fromCString(final MemorySegment addr) {
    return StringUtils.fromCString(addr);
  }

  /** Private constructor to prevent instantiation. */
  private HtmlToMarkdownFFI() {
    throw new UnsupportedOperationException("Utility class");
  }
}
