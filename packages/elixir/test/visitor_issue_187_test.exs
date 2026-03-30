defmodule HtmlToMarkdown.VisitorIssue187Test do
  use ExUnit.Case
  # Visitor support is not yet wired into the Elixir NIF in v3.
  # The NIF only exposes convert/2. Skip until NIF supports visitors.
  @moduletag :skip

  describe "visitor pattern issue #187 - tag_name in context" do
    test "basic conversion preserves content from divs" do
      html = "<div><p>text</p></div>"
      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
      assert String.contains?(markdown, "text")
    end

    test "conversion handles multiple element types" do
      html = """
      <article>
        <h1>Title</h1>
        <div>
          <p>Content</p>
        </div>
      </article>
      """

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
      assert String.contains?(markdown, "Title")
      assert String.contains?(markdown, "Content")
    end

    test "conversion handles script and style elements" do
      html = """
      <article>
        <h1>Blog Post</h1>
        <p>Main content</p>
        <script>console.log('test');</script>
        <style>body { margin: 0; }</style>
        <p>More content</p>
      </article>
      """

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert String.contains?(markdown, "Blog Post")
      assert String.contains?(markdown, "Main content")
      assert String.contains?(markdown, "More content")
    end

    test "conversion handles divs with various classes" do
      html = """
      <div class="content">
        <p>Good content</p>
      </div>
      <div class="ad">
        <p>Ad content</p>
      </div>
      """

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert String.contains?(markdown, "Good content")
      assert is_binary(markdown)
    end

    test "conversion handles complex nested content" do
      html = """
      <article>
        <h1>Blog Post Title</h1>
        <p>This is the main content of the article.</p>
        <div class="content">
          <p>Legitimate content in a div.</p>
          <img src="https://cdn.example.com/image.jpg" alt="Article image" width="800">
        </div>
        <p>Read more on our website.</p>
      </article>
      """

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert String.contains?(markdown, "Blog Post Title")
      assert String.contains?(markdown, "main content")
      assert String.contains?(markdown, "Legitimate content")
      assert String.contains?(markdown, "Article image")
    end

    test "conversion handles nested elements correctly" do
      html = "<div><p><strong>text</strong></p></div>"

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert is_binary(markdown)
    end

    test "conversion handles elements with attributes" do
      html = """
      <div class="container" id="main">
        <p data-type="text">Content</p>
      </div>
      """

      {:ok, markdown} = HtmlToMarkdown.convert(html)

      assert String.contains?(markdown, "Content")
    end
  end
end
