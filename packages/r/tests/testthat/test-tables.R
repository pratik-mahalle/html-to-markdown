test_that("convert handles a simple table", {
  html <- paste0(
    "<table><thead><tr><th>Name</th><th>Age</th></tr></thead>",
    "<tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>"
  )
  result <- convert(html)

  expect_type(result, "character")
  expect_true(grepl("Name", result))
  expect_true(grepl("Age", result))
  expect_true(grepl("Alice", result))
  expect_true(grepl("30", result))
})

test_that("convert handles non-table HTML", {
  html <- "<p>Hello world</p>"
  result <- convert(html)

  expect_type(result, "character")
})

test_that("convert handles multiple tables", {
  html <- paste0(
    "<table><tr><th>A</th></tr><tr><td>1</td></tr></table>",
    "<p>text</p>",
    "<table><tr><th>B</th></tr><tr><td>2</td></tr></table>"
  )
  result <- convert(html)

  expect_true(grepl("A", result))
  expect_true(grepl("B", result))
})

test_that("convert handles table with options", {
  html <- "<table><tr><th>H</th></tr><tr><td>V</td></tr></table>"
  opts <- conversion_options(heading_style = "atx")
  result <- convert(html, opts)

  expect_type(result, "character")
})

test_that("convert table content includes text", {
  html <- "<table><tr><th>Header</th></tr><tr><td>Value</td></tr></table>"
  result <- convert(html)

  expect_true(grepl("Header", result))
  expect_true(grepl("Value", result))
})
