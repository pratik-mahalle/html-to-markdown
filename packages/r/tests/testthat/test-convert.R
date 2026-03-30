test_that("convert basic HTML to markdown", {
  result <- convert("<h1>Hello</h1>")
  expect_type(result, "character")
  expect_match(result, "# Hello")
})

test_that("convert paragraph", {
  result <- convert("<p>Some text</p>")
  expect_match(result, "Some text")
})

test_that("convert multiple elements", {
  html <- "<h1>Title</h1><p>Paragraph</p><ul><li>Item 1</li><li>Item 2</li></ul>"
  result <- convert(html)
  expect_match(result, "# Title")
  expect_match(result, "Paragraph")
  expect_match(result, "Item 1")
  expect_match(result, "Item 2")
})

test_that("convert with options", {
  opts <- conversion_options(heading_style = "underlined")
  result <- convert("<h1>Underlined</h1>", opts)
  expect_match(result, "====")
})

test_that("convert with NULL options uses defaults", {
  result <- convert("<h1>Default</h1>", NULL)
  expect_match(result, "# Default")
})

test_that("convert with code block style", {
  opts <- conversion_options(code_block_style = "tildes")
  result <- convert("<pre><code>hello</code></pre>", opts)
  expect_match(result, "~~~")
})

test_that("convert with strip_tags", {
  opts <- conversion_options(strip_tags = c("div"))
  result <- convert("<div><p>Keep</p></div>", opts)
  expect_match(result, "Keep")
})

test_that("invalid option value raises error", {
  opts <- list(heading_style = "invalid_value")
  expect_error(convert("<h1>Test</h1>", opts))
})

test_that("convert empty string", {
  result <- convert("")
  expect_type(result, "character")
})

test_that("convert complex HTML", {
  html <- paste0(
    "<html><body>",
    "<h1>Title</h1>",
    "<p>A <strong>bold</strong> and <em>italic</em> text.</p>",
    "<a href=\"https://example.com\">Link</a>",
    "<table><tr><th>Header</th></tr><tr><td>Cell</td></tr></table>",
    "</body></html>"
  )
  result <- convert(html)
  expect_match(result, "# Title")
  expect_match(result, "\\*\\*bold\\*\\*")
  expect_match(result, "\\*italic\\*")
  expect_match(result, "\\[Link\\]")
})
