---
title: Java API Reference
description: API reference for the dev.kreuzberg html-to-markdown Java package
---

# Java API Reference <span class="version-badge new">v2.24.2</span>

**Package:** [`dev.kreuzberg:html-to-markdown`](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown) | **Version:** 2.26.0 | **Java:** 22+ (Panama FFI)

---

## Installation

=== "Maven"

    ```xml
    <dependency>
      <groupId>dev.kreuzberg</groupId>
      <artifactId>html-to-markdown</artifactId>
      <version>2.26.0</version>
    </dependency>
    ```

=== "Gradle"

    ```kotlin
    implementation("dev.kreuzberg:html-to-markdown:2.26.0")
    ```

The package uses Java's Foreign Function & Memory API (Panama) to call the native Rust library. No JNI code required.

---

## Class: `HtmlToMarkdown`

All methods are static on `dev.kreuzberg.htmltomarkdown.HtmlToMarkdown`.

### `convert`

Convert HTML to Markdown using default options.

```java
public static String convert(String html)
public static String convert(String html, ConversionOptions options)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `String` | The HTML string to convert |
| `options` | `ConversionOptions` | Optional conversion options |

**Returns:** `String` -- the converted Markdown.

**Throws:** `HtmlToMarkdownException` on conversion failure.

**Example:**

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.ConversionOptions;

String html = "<h1>Hello</h1><p>World</p>";
String markdown = HtmlToMarkdown.convert(html);

// With options
ConversionOptions options = ConversionOptions.builder()
    .headingStyle("atx")
    .codeBlockStyle("backticks")
    .wrap(true)
    .wrapWidth(80)
    .build();
String markdown = HtmlToMarkdown.convert(html, options);
```

---

### `convertWithMetadata`

Convert HTML to Markdown with metadata extraction.

```java
public static ConversionResult convertWithMetadata(String html)
public static ConversionResult convertWithMetadata(String html, ConversionOptions options)
```

**Returns:** `ConversionResult` -- object containing `getMarkdown()` and `getMetadata()`.

**Example:**

```java
ConversionResult result = HtmlToMarkdown.convertWithMetadata(html);
System.out.println(result.getMarkdown());

ExtendedMetadata metadata = result.getMetadata();
System.out.println(metadata.getDocument().getTitle());
System.out.println(metadata.getHeaders().size());
System.out.println(metadata.getLinks().size());
```

---

### `convertWithVisitor`

Convert HTML with a custom visitor.

```java
public static String convertWithVisitor(String html, Visitor visitor)
public static String convertWithVisitor(String html, Visitor visitor, ConversionOptions options)
```

**Example:**

```java
import dev.kreuzberg.htmltomarkdown.visitor.Visitor;
import dev.kreuzberg.htmltomarkdown.visitor.VisitResult;

Visitor visitor = new Visitor() {
    @Override
    public VisitResult visitImage(NodeContext ctx, String src, String alt, String title) {
        return VisitResult.skip();
    }
};

String markdown = HtmlToMarkdown.convertWithVisitor(html, visitor);
```

---

### `version`

Return the version string of the native library.

```java
public static String version()
```

---

## Classes

### `ConversionOptions`

Built using the builder pattern.

```java
ConversionOptions options = ConversionOptions.builder()
    .headingStyle("atx")          // "underlined", "atx", "atxClosed"
    .listIndentType("spaces")     // "spaces", "tabs"
    .listIndentWidth(2)
    .bullets("-")
    .codeBlockStyle("backticks")  // "indented", "backticks", "tildes"
    .wrap(true)
    .wrapWidth(80)
    .preserveTags(List.of("table", "div"))
    .skipImages(false)
    .outputFormat("markdown")     // "markdown", "djot"
    .build();
```

See the [Configuration Reference](configuration.md) for all fields.

---

### `Visitor` Interface

```java
public interface Visitor {
    default VisitResult visitText(NodeContext ctx, String text) { return VisitResult.continueResult(); }
    default VisitResult visitLink(NodeContext ctx, String href, String text, String title) { return VisitResult.continueResult(); }
    default VisitResult visitImage(NodeContext ctx, String src, String alt, String title) { return VisitResult.continueResult(); }
    default VisitResult visitHeading(NodeContext ctx, int level, String text, String id) { return VisitResult.continueResult(); }
    default VisitResult visitCodeBlock(NodeContext ctx, String language, String code) { return VisitResult.continueResult(); }
    default VisitResult visitCodeInline(NodeContext ctx, String code) { return VisitResult.continueResult(); }
    default VisitResult visitListItem(NodeContext ctx, boolean ordered, String marker, String text) { return VisitResult.continueResult(); }
    default VisitResult visitTableRow(NodeContext ctx, String[] cells, boolean isHeader) { return VisitResult.continueResult(); }
    default VisitResult visitBlockquote(NodeContext ctx, String content, int depth) { return VisitResult.continueResult(); }
    default VisitResult visitStrong(NodeContext ctx, String text) { return VisitResult.continueResult(); }
    default VisitResult visitEmphasis(NodeContext ctx, String text) { return VisitResult.continueResult(); }
    default VisitResult visitElementStart(NodeContext ctx) { return VisitResult.continueResult(); }
    default VisitResult visitElementEnd(NodeContext ctx, String output) { return VisitResult.continueResult(); }
    // ... and more
}
```

### `VisitResult`

```java
public class VisitResult {
    public static VisitResult continueResult();
    public static VisitResult skip();
    public static VisitResult preserveHtml();
    public static VisitResult custom(String output);
    public static VisitResult error(String message);
}
```

---

## Panama FFI Details

The Java binding uses the Foreign Function & Memory API (Project Panama) introduced in Java 22. Key implementation details:

- Native library loaded via `SymbolLookup` and `Linker.nativeLinker()`
- Memory managed through `Arena` scopes for automatic cleanup
- String marshalling uses UTF-8 encoding via `MemorySegment`
- No JNI code, header files, or generated stubs required

**JVM flags** (if needed):

```
--enable-native-access=ALL-UNNAMED
```

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
