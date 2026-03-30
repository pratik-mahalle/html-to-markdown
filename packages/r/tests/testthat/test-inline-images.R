test_that("convert handles HTML without images", {
  html <- "<p>No images here</p>"
  result <- convert(html)
  expect_type(result, "character")
  expect_match(result, "No images here")
})

test_that("convert with NULL options handles images", {
  html <- "<p>Test</p>"
  result <- convert(html, NULL)
  expect_type(result, "character")
})

test_that("convert handles HTML with base64 image", {
  # A minimal 1x1 red pixel PNG as base64
  png_base64 <- paste0(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8",
    "/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg=="
  )
  html <- paste0("<img src=\"data:image/png;base64,", png_base64, "\" alt=\"Red pixel\">")
  result <- convert(html)
  expect_type(result, "character")
})
