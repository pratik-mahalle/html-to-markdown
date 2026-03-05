test_that("convert_with_tables extracts a simple table", {
  html <- paste0(
    "<table><thead><tr><th>Name</th><th>Age</th></tr></thead>",
    "<tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>"
  )
  result <- convert_with_tables(html)

  expect_type(result, "list")
  expect_true("content" %in% names(result))
  expect_true("tables" %in% names(result))
  expect_type(result$content, "character")
  expect_length(result$tables, 1)

  table <- result$tables[[1]]
  expect_true("cells" %in% names(table))
  expect_true("markdown" %in% names(table))
  expect_true("is_header_row" %in% names(table))
  expect_equal(table$cells[[1]], c("Name", "Age"))
  expect_equal(table$cells[[2]], c("Alice", "30"))
  expect_true(table$is_header_row[[1]])
})

test_that("convert_with_tables returns empty tables for non-table HTML", {
  html <- "<p>Hello world</p>"
  result <- convert_with_tables(html)

  expect_type(result$content, "character")
  expect_length(result$tables, 0)
})

test_that("convert_with_tables extracts multiple tables", {
  html <- paste0(
    "<table><tr><th>A</th></tr><tr><td>1</td></tr></table>",
    "<p>text</p>",
    "<table><tr><th>B</th></tr><tr><td>2</td></tr></table>"
  )
  result <- convert_with_tables(html)

  expect_length(result$tables, 2)
})

test_that("convert_with_tables includes metadata", {
  html <- paste0(
    "<html><head><title>Test</title></head><body>",
    "<table><tr><th>Col</th></tr><tr><td>Val</td></tr></table>",
    "</body></html>"
  )
  result <- convert_with_tables(html)

  expect_true("metadata" %in% names(result))
})

test_that("convert_with_tables accepts options", {
  html <- "<table><tr><th>H</th></tr><tr><td>V</td></tr></table>"
  result <- convert_with_tables(html, options = list(heading_style = "atx"))

  expect_length(result$tables, 1)
})

test_that("convert_with_tables content includes table text", {
  html <- "<table><tr><th>Header</th></tr><tr><td>Value</td></tr></table>"
  result <- convert_with_tables(html)

  expect_true(grepl("Header", result$content))
  expect_true(grepl("Value", result$content))
})
