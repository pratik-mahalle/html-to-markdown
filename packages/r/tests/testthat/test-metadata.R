test_that("convert returns markdown for HTML with metadata", {
  html <- "<html><head><title>My Page</title></head><body><h1>Header</h1></body></html>"
  result <- convert(html)
  expect_type(result, "character")
  expect_match(result, "# Header")
})

test_that("convert handles HTML with meta tags", {
  html <- paste0(
    "<html><head>",
    "<title>Test Title</title>",
    "<meta name=\"description\" content=\"Test description\">",
    "<meta name=\"author\" content=\"Test Author\">",
    "</head><body><p>Content</p></body></html>"
  )
  result <- convert(html)
  expect_type(result, "character")
  expect_match(result, "Content")
})

test_that("convert handles HTML with headers", {
  html <- "<h1>H1</h1><h2>H2</h2><h3>H3</h3>"
  result <- convert(html)
  expect_match(result, "# H1")
  expect_match(result, "## H2")
  expect_match(result, "### H3")
})

test_that("convert handles HTML with links", {
  html <- "<a href=\"https://example.com\">Example</a><a href=\"https://test.com\">Test</a>"
  result <- convert(html)
  expect_type(result, "character")
})

test_that("convert with NULL options", {
  html <- "<h1>Test</h1>"
  result <- convert(html, NULL)
  expect_type(result, "character")
  expect_match(result, "# Test")
})
