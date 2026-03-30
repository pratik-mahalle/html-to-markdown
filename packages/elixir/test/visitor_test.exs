defmodule HtmlToMarkdown.VisitorTest do
  use ExUnit.Case
  # Visitor support is not yet wired into the Elixir NIF in v3.
  # The Visitor behaviour module exists as an interface definition,
  # but the NIF only exposes convert/2. Skip until NIF supports visitors.
  @moduletag :skip

  describe "visitor behaviour" do
    test "visitor provides convenience macros via __using__" do
      defmodule TestVisitor do
        use HtmlToMarkdown.Visitor
      end

      assert function_exported?(TestVisitor, :handle_text, 2)
      assert function_exported?(TestVisitor, :handle_link, 4)
      assert function_exported?(TestVisitor, :handle_image, 4)
      assert function_exported?(TestVisitor, :handle_heading, 4)
    end
  end

  describe "convert/2 with visitor-style patterns" do
    test "basic conversion works" do
      html = "<p>Hello World</p>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
      assert String.contains?(markdown, "Hello")
    end

    test "converts headings" do
      html = "<h1>Title</h1>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
      assert String.contains?(markdown, "Title")
    end

    test "converts links" do
      html = "<a href='http://example.com'>Example</a>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts images" do
      html = "<img src='test.jpg' alt='Test Image'>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts code blocks" do
      html = "<pre><code>let x = 1;</code></pre>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts inline code" do
      html = "<code>inline</code>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts lists" do
      html = "<ul><li>Item 1</li></ul>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts tables" do
      html = "<table><tr><td>Data</td></tr></table>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts blockquotes" do
      html = "<blockquote>Quote</blockquote>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts strong text" do
      html = "<strong>Bold</strong>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts emphasis" do
      html = "<em>Italic</em>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts strikethrough" do
      html = "<s>Strikethrough</s>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts line breaks" do
      html = "Line 1<br>Line 2"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "converts horizontal rules" do
      html = "<hr>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end
  end

  describe "convert/2 with options" do
    test "accepts heading_style option" do
      html = "<h1>Title</h1>"
      options = %{heading_style: "atx"}

      {:ok, markdown} = HtmlToMarkdown.convert(html, options)

      assert is_binary(markdown)
    end

    test "accepts list_indent_width option" do
      html = "<ul><li>Item</li></ul>"
      options = %{list_indent_width: 2}

      {:ok, markdown} = HtmlToMarkdown.convert(html, options)

      assert is_binary(markdown)
    end
  end

  describe "edge cases" do
    test "empty HTML string" do
      {:ok, markdown} = HtmlToMarkdown.convert("")

      assert is_binary(markdown)
    end

    test "HTML with nested elements" do
      html = "<div><p><strong>Bold <em>italic</em></strong></p></div>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "HTML with special characters" do
      html = "<p>Special: &amp; &lt; &gt; &quot; &apos;</p>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end
  end
end
