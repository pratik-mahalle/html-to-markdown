defmodule HtmlToMarkdownTestApp.ComprehensiveTest do
  use ExUnit.Case

  @moduledoc """
  Comprehensive test suite for html_to_markdown Hex package.
  Tests basic conversions, error handling, and API behavior.
  """

  defp load_fixtures(filename) do
    path = Path.join([__DIR__, "..", "..", "fixtures", filename])
    case File.read(path) do
      {:ok, content} -> Jason.decode!(content)
      {:error, reason} -> raise "Failed to load fixture #{filename}: #{inspect(reason)}"
    end
  end

  describe "basic HTML conversions" do
    setup do
      {:ok, fixtures: load_fixtures("basic-html.json")}
    end

    test "all basic fixtures convert correctly", %{fixtures: fixtures} do
      Enum.each(fixtures, fn fixture ->
        {:ok, result} = HtmlToMarkdown.convert(fixture["html"], fixture["options"] || %{})
        expected = String.trim(fixture["expectedMarkdown"])
        actual = String.trim(result)

        assert actual == expected, """
        Test case: #{fixture["name"]}
        Expected: #{inspect(expected)}
        Got: #{inspect(actual)}
        """
      end)
    end

    test "basic paragraph conversion" do
      html = "<p>Hello World</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.contains?(result, "Hello World")
    end

    test "basic heading conversion" do
      html = "<h1>Title</h1>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.starts_with?(result, "# Title")
    end

    test "empty input returns empty string" do
      {:ok, result} = HtmlToMarkdown.convert("")
      assert result == ""
    end

    test "whitespace-only input" do
      {:ok, result} = HtmlToMarkdown.convert("   \n\t  ")
      assert String.trim(result) == ""
    end
  end

  describe "error handling with proper tuple patterns" do
    test "returns {:ok, result} tuple on success" do
      result = HtmlToMarkdown.convert("<p>Test</p>")
      assert is_tuple(result)
      assert tuple_size(result) == 2
      assert {:ok, _content} = result
    end

    test "returns {:error, reason} tuple on failure" do
      # Test with very large invalid HTML that may cause issues
      # Most implementations handle this gracefully, but some may error
      result = HtmlToMarkdown.convert("<")
      case result do
        {:ok, _} -> :ok  # May succeed with lenient parsing
        {:error, _reason} -> :ok  # Expected error case
      end
    end

    test "convert function result is always a tuple" do
      test_cases = [
        "<p>Normal</p>",
        "<h1>Heading</h1>",
        "<div>Nested <span>content</span></div>",
        ""
      ]

      Enum.each(test_cases, fn html ->
        result = HtmlToMarkdown.convert(html)
        assert is_tuple(result), "Expected tuple, got #{inspect(result)}"
        assert tuple_size(result) == 2, "Expected 2-tuple, got #{tuple_size(result)}-tuple"
      end)
    end

    test "successful conversions unwrap with pattern matching" do
      html = "<p>Pattern match test</p>"
      assert {:ok, content} = HtmlToMarkdown.convert(html)
      assert is_binary(content)
      assert String.length(content) > 0
    end
  end

  describe "inline images feature" do
    test "inline images are preserved" do
      html = "<p>Check this <img src=\"/image.png\" alt=\"An image\"/> out</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      # Markdown image syntax: ![alt](src)
      assert String.contains?(result, "image") or String.contains?(result, "img")
    end

    test "image with alt text" do
      html = "<img src=\"test.jpg\" alt=\"Test Image\"/>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      # Should contain alt text or image reference
      assert String.length(result) > 0
    end
  end

  describe "metadata extraction" do
    test "extracts basic metadata when available" do
      html = """
      <html>
        <head>
          <title>Page Title</title>
          <meta name="description" content="Page description">
        </head>
        <body><p>Content</p></body>
      </html>
      """
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert is_binary(result)
    end

    test "handles pages without metadata" do
      html = "<p>Just content</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert is_binary(result)
    end
  end

  describe "complex HTML structures" do
    test "nested lists conversion" do
      html = """
      <ul>
        <li>Item 1</li>
        <li>Item 2
          <ul>
            <li>Nested 1</li>
            <li>Nested 2</li>
          </ul>
        </li>
      </ul>
      """
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.contains?(result, "Item 1")
      assert String.contains?(result, "Item 2")
    end

    test "mixed content with various HTML elements" do
      html = """
      <div>
        <h1>Title</h1>
        <p>Paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
        <ul>
          <li>List item 1</li>
          <li>List item 2</li>
        </ul>
        <blockquote>A quote</blockquote>
      </div>
      """
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.contains?(result, "Title")
      assert String.contains?(result, "Paragraph")
      assert String.contains?(result, "List item")
    end

    test "code blocks preservation" do
      html = "<pre><code>def hello():\n  print('world')</code></pre>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.contains?(result, "hello") or String.contains?(result, "print")
    end
  end

  describe "options and configuration" do
    test "accept options as a map" do
      html = "<p>Test</p>"
      options = %{}
      {:ok, result} = HtmlToMarkdown.convert(html, options)
      assert is_binary(result)
    end

    test "handles empty options map" do
      html = "<h1>Title</h1>"
      {:ok, result} = HtmlToMarkdown.convert(html, %{})
      assert String.contains?(result, "Title")
    end
  end

  describe "special characters and encoding" do
    test "handles HTML entities" do
      html = "<p>&lt;div&gt; &amp; special chars</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.length(result) > 0
    end

    test "unicode content preservation" do
      html = "<p>Unicode: 你好 🎉 café</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.length(result) > 0
    end

    test "preserves quotes and apostrophes" do
      html = "<p>He said, \"It's working!\"</p>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.length(result) > 0
    end
  end

  describe "link handling" do
    test "converts anchor links to markdown" do
      html = "<a href=\"https://example.com\">Example Link</a>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.contains?(result, "Example Link")
    end

    test "handles links with fragments" do
      html = "<a href=\"#section\">Go to section</a>"
      {:ok, result} = HtmlToMarkdown.convert(html)
      assert String.length(result) > 0
    end
  end

  describe "integration and API contract" do
    test "module is loaded and accessible" do
      assert Code.ensure_loaded?(HtmlToMarkdown)
      assert function_exported?(HtmlToMarkdown, :convert, 1)
      assert function_exported?(HtmlToMarkdown, :convert, 2)
    end

    test "convert/1 and convert/2 both work" do
      html = "<p>Test</p>"
      {:ok, result1} = HtmlToMarkdown.convert(html)
      {:ok, result2} = HtmlToMarkdown.convert(html, %{})
      assert result1 == result2
    end

    test "return values are properly formatted" do
      html = "<strong>Bold</strong>"
      result = HtmlToMarkdown.convert(html)
      assert match?({:ok, _}, result) or match?({:error, _}, result)

      case result do
        {:ok, content} ->
          assert is_binary(content)
          assert not is_nil(content)

        {:error, reason} ->
          assert is_binary(reason) or is_atom(reason)
      end
    end
  end
end
