using System.Runtime.InteropServices;

namespace HtmlToMarkdown;

/// <summary>
/// P/Invoke declarations for the native html-to-markdown library.
/// </summary>
internal static class NativeMethods
{
    private const string LibraryName = "html_to_markdown_ffi";

    /// <summary>
    /// Free a string returned by html_to_markdown_convert.
    /// </summary>
    /// <param name="ptr">Pointer to string to free</param>
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void html_to_markdown_free_string(IntPtr ptr);

    /// <summary>
    /// Get the last error message.
    /// </summary>
    /// <returns>Pointer to static null-terminated error string, or NULL</returns>
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    internal static extern IntPtr html_to_markdown_last_error();

    /// <summary>
    /// Convert HTML and return structured content: markdown, metadata, tables, and warnings as JSON.
    /// </summary>
    /// <param name="html">Null-terminated HTML string</param>
    /// <param name="options_json">Null-terminated JSON string for conversion options, or NULL</param>
    /// <returns>Pointer to null-terminated JSON string, or NULL on error</returns>
    /// <remarks>
    /// The returned JSON string must be freed with html_to_markdown_free_string.
    /// </remarks>
    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    internal static extern IntPtr html_to_markdown_convert(IntPtr html, IntPtr options_json);
}
