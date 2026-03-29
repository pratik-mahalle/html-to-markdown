using System.Runtime.InteropServices;
using System.Text.Json;
using HtmlToMarkdown.Metadata;
using HtmlToMarkdown.Serialization;

namespace HtmlToMarkdown;

/// <summary>
/// High-performance HTML to Markdown converter with .NET bindings to Rust core.
/// </summary>
public static class HtmlToMarkdownConverter
{
    /// <summary>
    /// Converts HTML and returns full structured content in a single pass.
    /// </summary>
    /// <param name="html">The HTML string to convert</param>
    /// <param name="optionsJson">Optional JSON string for conversion options, or null for defaults</param>
    /// <returns>
    /// A <see cref="Metadata.ConversionResult"/> containing the converted Markdown, extracted
    /// metadata (title, links, images, etc.), structured table data, and any processing warnings.
    /// </returns>
    /// <exception cref="ArgumentNullException">Thrown when html is null</exception>
    /// <exception cref="HtmlToMarkdownException">Thrown when conversion or JSON parsing fails</exception>
    public static Metadata.ConversionResult Convert(string html, string? optionsJson = null)
    {
        if (html == null)
        {
            throw new ArgumentNullException(nameof(html));
        }

        if (string.IsNullOrEmpty(html))
        {
            return new Metadata.ConversionResult();
        }

        IntPtr htmlPtr = IntPtr.Zero;
        IntPtr optionsPtr = IntPtr.Zero;
        IntPtr resultPtr = IntPtr.Zero;

        try
        {
            htmlPtr = StringToUtf8Ptr(html);
            if (optionsJson != null)
            {
                optionsPtr = StringToUtf8Ptr(optionsJson);
            }

            resultPtr = NativeMethods.html_to_markdown_convert(htmlPtr, optionsPtr);

            if (resultPtr == IntPtr.Zero)
            {
                IntPtr errorPtr = NativeMethods.html_to_markdown_last_error();
                string? errorMsg = errorPtr != IntPtr.Zero
                    ? PtrToStringUtf8(errorPtr)
                    : null;

                throw new HtmlToMarkdownException(
                    errorMsg ?? "HTML conversion failed");
            }

            string jsonStr = PtrToStringUtf8(resultPtr) ?? "{}";
            return JsonSerializer.Deserialize(jsonStr, MetadataJsonContext.Default.ConversionResult)
                ?? new Metadata.ConversionResult();
        }
        catch (JsonException ex)
        {
            throw new HtmlToMarkdownException(
                $"Failed to deserialize conversion JSON: {ex.Message}", ex);
        }
        finally
        {
            if (htmlPtr != IntPtr.Zero)
            {
                Marshal.FreeCoTaskMem(htmlPtr);
            }

            if (optionsPtr != IntPtr.Zero)
            {
                Marshal.FreeCoTaskMem(optionsPtr);
            }

            if (resultPtr != IntPtr.Zero)
            {
                NativeMethods.html_to_markdown_free_string(resultPtr);
            }
        }
    }

    private static string? PtrToStringUtf8(IntPtr ptr)
    {
        return ptr == IntPtr.Zero ? null : Marshal.PtrToStringUTF8(ptr);
    }

    private static IntPtr StringToUtf8Ptr(string value)
    {
        return Marshal.StringToCoTaskMemUTF8(value);
    }
}

/// <summary>
/// Exception thrown when HTML to Markdown conversion fails.
/// </summary>
public class HtmlToMarkdownException : Exception
{
    public HtmlToMarkdownException(string message) : base(message) { }

    public HtmlToMarkdownException(string message, Exception innerException)
        : base(message, innerException) { }
}
