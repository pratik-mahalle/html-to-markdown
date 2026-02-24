test_that("convert_with_inline_images returns expected structure", {
  html <- "<p>No images here</p>"
  result <- convert_with_inline_images(html)
  expect_type(result, "list")
  expect_true("markdown" %in% names(result))
  expect_true("images" %in% names(result))
  expect_true("warnings" %in% names(result))
  expect_match(result$markdown, "No images here")
})

test_that("convert_with_inline_images with NULL options", {
  html <- "<p>Test</p>"
  result <- convert_with_inline_images(html, options = NULL, config = NULL)
  expect_type(result, "list")
})

test_that("convert_with_inline_images with base64 image", {
  # A minimal 1x1 red pixel PNG as base64
  png_base64 <- paste0(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8",
    "/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=="
  )
  html <- paste0("<img src=\"data:image/png;base64,", png_base64, "\" alt=\"Red pixel\">")
  result <- convert_with_inline_images(html)
  expect_type(result, "list")
})
