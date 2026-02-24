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
  result <- convert_with_options("<h1>Underlined</h1>", opts)
  expect_match(result, "====")
})

test_that("convert with NULL options uses defaults", {
  result <- convert_with_options("<h1>Default</h1>", NULL)
  expect_match(result, "# Default")
})

test_that("convert with options handle", {
  opts <- conversion_options(heading_style = "atx_closed")
  handle <- create_options_handle(opts)
  result <- convert_with_options_handle("<h1>Closed</h1>", handle)
  expect_match(result, "# Closed #")
})

test_that("options handle can be reused", {
  opts <- conversion_options(heading_style = "underlined")
  handle <- create_options_handle(opts)
  r1 <- convert_with_options_handle("<h1>First</h1>", handle)
  r2 <- convert_with_options_handle("<h1>Second</h1>", handle)
  expect_match(r1, "====")
  expect_match(r2, "====")
})

test_that("convert with code block style", {
  opts <- conversion_options(code_block_style = "tildes")
  result <- convert_with_options("<pre><code>hello</code></pre>", opts)
  expect_match(result, "~~~")
})

test_that("convert with strip_tags", {
  opts <- conversion_options(strip_tags = c("div"))
  result <- convert_with_options("<div><p>Keep</p></div>", opts)
  expect_match(result, "Keep")
})

test_that("invalid option value raises error", {
  opts <- list(heading_style = "invalid_value")
  expect_error(convert_with_options("<h1>Test</h1>", opts))
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
