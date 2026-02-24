test_that("convert_with_metadata returns markdown and metadata", {
  html <- "<html><head><title>My Page</title></head><body><h1>Header</h1></body></html>"
  result <- convert_with_metadata(html)
  expect_type(result, "list")
  expect_true("markdown" %in% names(result))
  expect_true("metadata" %in% names(result))
  expect_match(result$markdown, "# Header")
})

test_that("metadata contains document info", {
  html <- paste0(
    "<html><head>",
    "<title>Test Title</title>",
    "<meta name=\"description\" content=\"Test description\">",
    "<meta name=\"author\" content=\"Test Author\">",
    "</head><body><p>Content</p></body></html>"
  )
  result <- convert_with_metadata(html)
  doc <- result$metadata$document
  expect_equal(doc$title, "Test Title")
  expect_equal(doc$description, "Test description")
  expect_equal(doc$author, "Test Author")
})

test_that("metadata extracts headers", {
  html <- "<h1>H1</h1><h2>H2</h2><h3>H3</h3>"
  result <- convert_with_metadata(html)
  headers <- result$metadata$headers
  expect_true(length(headers) >= 3)
})

test_that("metadata extracts links", {
  html <- "<a href=\"https://example.com\">Example</a><a href=\"https://test.com\">Test</a>"
  result <- convert_with_metadata(html)
  links <- result$metadata$links
  expect_true(length(links) >= 2)
})

test_that("metadata config can limit extraction", {
  html <- "<html><head><title>Title</title></head><body><h1>H</h1><a href=\"#\">L</a></body></html>"
  config <- list(
    extract_document = TRUE,
    extract_headers = FALSE,
    extract_links = FALSE,
    extract_images = FALSE,
    extract_structured_data = FALSE
  )
  result <- convert_with_metadata(html, config = config)
  expect_equal(result$metadata$document$title, "Title")
})

test_that("convert_with_metadata with NULL options", {
  html <- "<h1>Test</h1>"
  result <- convert_with_metadata(html, options = NULL, config = NULL)
  expect_type(result, "list")
  expect_match(result$markdown, "# Test")
})
