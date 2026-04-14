defmodule HtmlToMarkdownTestApp.SmokeTest do
  use ExUnit.Case

  @moduletag :smoke

  @moduledoc """
  Smoke tests for html_to_markdown Hex package.
  Validates that the published package loads and works correctly.
  """

  test "package loads from Hex.pm" do
    assert Code.ensure_loaded?(HtmlToMarkdown)
  end

  test "basic conversion works" do
    html = "<p>Hello World</p>"
    {:ok, result} = HtmlToMarkdown.convert(html)
    assert String.contains?(result, "Hello World")
  end

  test "heading conversion works" do
    html = "<h1>Title</h1>"
    {:ok, result} = HtmlToMarkdown.convert(html)
    assert String.starts_with?(result, "#")
  end

  test "empty input returns empty string" do
    {:ok, result} = HtmlToMarkdown.convert("")
    assert result == ""
  end

  test "convert/1 returns {:ok, binary} tuple" do
    result = HtmlToMarkdown.convert("<p>Test</p>")
    assert is_tuple(result)
    assert match?({:ok, _binary}, result)
  end

  test "convert/2 with options works" do
    html = "<p>Content</p>"
    {:ok, result1} = HtmlToMarkdown.convert(html)
    {:ok, result2} = HtmlToMarkdown.convert(html, %{})
    assert result1 == result2
  end

  test "published package version is accessible" do
    # Verify the package info is accessible
    assert Code.ensure_loaded?(HtmlToMarkdown)
    # All required functions should be exported
    assert function_exported?(HtmlToMarkdown, :convert, 1)
    assert function_exported?(HtmlToMarkdown, :convert, 2)
  end

  test "various HTML elements convert without errors" do
    test_cases = [
      "<p>Text</p>",
      "<strong>Bold</strong>",
      "<em>Italic</em>",
      "<a href=\"#\">Link</a>",
      "<ul><li>List</li></ul>",
      "<code>code</code>",
      "<blockquote>Quote</blockquote>"
    ]

    Enum.each(test_cases, fn html ->
      result = HtmlToMarkdown.convert(html)
      assert match?({:ok, _}, result),
        "Failed to convert: #{html}, got: #{inspect(result)}"
    end)
  end
end
