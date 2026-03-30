defmodule HtmlToMarkdown.TablesTest do
  use ExUnit.Case, async: true

  describe "convert/2 with tables" do
    test "converts a simple table" do
      html = """
      <table>
        <thead><tr><th>Name</th><th>Age</th></tr></thead>
        <tbody><tr><td>Alice</td><td>30</td></tr></tbody>
      </table>
      """

      {:ok, content} = HtmlToMarkdown.convert(html)
      assert is_binary(content)
      assert String.contains?(content, "Name")
      assert String.contains?(content, "Age")
      assert String.contains?(content, "Alice")
      assert String.contains?(content, "30")
    end

    test "converts non-table HTML without error" do
      html = "<p>Hello world</p>"
      {:ok, content} = HtmlToMarkdown.convert(html)
      assert is_binary(content)
    end

    test "converts multiple tables" do
      html = """
      <table><tr><th>A</th></tr><tr><td>1</td></tr></table>
      <p>Text between</p>
      <table><tr><th>B</th></tr><tr><td>2</td></tr></table>
      """

      {:ok, content} = HtmlToMarkdown.convert(html)
      assert String.contains?(content, "A")
      assert String.contains?(content, "B")
    end

    test "content includes table markdown" do
      html = "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>"
      {:ok, content} = HtmlToMarkdown.convert(html)
      assert String.contains?(content, "X")
      assert String.contains?(content, "Y")
    end

    test "handles special characters in cells" do
      html = "<table><tr><td>c &amp; d</td></tr></table>"
      {:ok, content} = HtmlToMarkdown.convert(html)
      assert is_binary(content)
    end

    test "accepts options" do
      html = "<table><tr><th>H</th></tr><tr><td>V</td></tr></table>"

      {:ok, content} = HtmlToMarkdown.convert(html, %{heading_style: "atx"})
      assert is_binary(content)
    end
  end
end
