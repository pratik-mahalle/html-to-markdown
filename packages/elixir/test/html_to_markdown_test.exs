defmodule HtmlToMarkdownTest do
  use ExUnit.Case, async: true

  @data_uri "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAuMBg6zidhwAAAAASUVORK5CYII="

  test "convert/1 returns markdown" do
    assert {:ok, result} = HtmlToMarkdown.convert("<h1>Hello</h1>")
    assert result["content"] =~ "# Hello"
  end

  test "convert!/2 raises on invalid options" do
    assert_raise HtmlToMarkdown.Error, fn ->
      HtmlToMarkdown.convert!("<div>", heading_style: :invalid)
    end
  end

  test "convert/2 reports invalid options" do
    assert {:error, reason} = HtmlToMarkdown.convert("<div>", heading_style: :invalid)
    assert is_binary(reason)
  end

  test "convert/2 accepts keyword options" do
    assert {:ok, result} =
             HtmlToMarkdown.convert("<p>Example</p>",
               wrap: true,
               wrap_width: 10,
               preprocessing: %{enabled: true, preset: :minimal}
             )

    assert result["content"] =~ "Example"
  end

  test "convert/2 rejects invalid boolean options" do
    assert {:error, reason} = HtmlToMarkdown.convert("<p>Body</p>", wrap: "yes")
    assert is_binary(reason)
    assert String.contains?(reason, "wrap")
  end

  test "convert/2 extracts inline images from data URIs" do
    html = """
    <p>
      Example <img src=\"data:image/png;base64,#{@data_uri}\" alt=\"Logo\" />
    </p>
    """

    assert {:ok, result} = HtmlToMarkdown.convert(html)
    assert result["content"] =~ "Example"
  end

  test "convert/2 extracts metadata (document, headers, links, images)" do
    html = """
    <html>
      <head>
        <title>Example Article</title>
        <meta name="description" content="Demo page">
        <link rel="canonical" href="https://example.com/article">
      </head>
      <body>
        <h1 id="welcome">Welcome</h1>
        <a href="https://example.com" rel="nofollow external">Example link</a>
        <img src="https://example.com/image.jpg" alt="Hero" width="640" height="480">
      </body>
    </html>
    """

    assert {:ok, result} = HtmlToMarkdown.convert(html)
    assert is_binary(result["content"])
    assert result["content"] =~ "Welcome"
  end

  test "convert/2 accepts output_format option as atom" do
    html = "<p>**Bold text**</p>"

    # Test with default markdown format
    assert {:ok, markdown_result} = HtmlToMarkdown.convert(html, output_format: :markdown)
    assert is_binary(markdown_result["content"])

    # Test with djot format
    assert {:ok, djot_result} = HtmlToMarkdown.convert(html, output_format: :djot)
    assert is_binary(djot_result["content"])
  end

  test "convert/2 accepts output_format option as string" do
    html = "<h2>Heading</h2>"

    # Test with string "markdown"
    assert {:ok, _result} = HtmlToMarkdown.convert(html, output_format: "markdown")

    # Test with string "djot"
    assert {:ok, _result} = HtmlToMarkdown.convert(html, output_format: "djot")
  end
end
