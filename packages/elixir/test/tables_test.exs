defmodule HtmlToMarkdown.TablesTest do
  use ExUnit.Case, async: true

  describe "convert_with_tables/3" do
    test "extracts a simple table" do
      html = """
      <table>
        <thead><tr><th>Name</th><th>Age</th></tr></thead>
        <tbody><tr><td>Alice</td><td>30</td></tr></tbody>
      </table>
      """

      assert {:ok, content, tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert is_binary(content)
      assert length(tables) == 1

      [table] = tables
      assert is_list(table.cells)
      assert length(table.cells) >= 2
      assert hd(table.cells) == ["Name", "Age"]
      assert Enum.at(table.cells, 1) == ["Alice", "30"]
      assert is_binary(table.markdown)
      assert is_list(table.is_header_row)
      assert hd(table.is_header_row) == true
    end

    test "returns empty tables for non-table HTML" do
      html = "<p>Hello world</p>"
      assert {:ok, content, tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert is_binary(content)
      assert tables == []
    end

    test "extracts multiple tables" do
      html = """
      <table><tr><th>A</th></tr><tr><td>1</td></tr></table>
      <p>Text between</p>
      <table><tr><th>B</th></tr><tr><td>2</td></tr></table>
      """

      assert {:ok, _content, tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert length(tables) == 2
    end

    test "includes metadata when present" do
      html = """
      <html><head><title>Test</title></head>
      <body><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table></body></html>
      """

      assert {:ok, _content, _tables, metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert is_map(metadata)
      assert Map.has_key?(metadata, "document")
    end

    test "content includes table markdown" do
      html = "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>"
      assert {:ok, content, _tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert String.contains?(content, "X")
      assert String.contains?(content, "Y")
    end

    test "handles special characters in cells" do
      html = "<table><tr><td>a | b</td><td>c &amp; d</td></tr></table>"
      assert {:ok, _content, tables, _metadata} = HtmlToMarkdown.convert_with_tables(html)
      assert length(tables) == 1
      [table] = tables
      assert hd(table.cells) == ["a | b", "c & d"]
    end

    test "accepts options" do
      html = "<table><tr><th>H</th></tr><tr><td>V</td></tr></table>"

      assert {:ok, _content, tables, _metadata} =
               HtmlToMarkdown.convert_with_tables(html, heading_style: :atx)

      assert length(tables) == 1
    end
  end

  describe "convert_with_tables!/3" do
    test "returns result tuple on success" do
      html = "<table><tr><th>A</th></tr><tr><td>1</td></tr></table>"
      {content, tables, _metadata} = HtmlToMarkdown.convert_with_tables!(html)
      assert is_binary(content)
      assert length(tables) == 1
    end
  end
end
