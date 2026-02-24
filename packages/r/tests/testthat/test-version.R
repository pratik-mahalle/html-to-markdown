test_that("version returns a string", {
  v <- version()
  expect_type(v, "character")
  expect_match(v, "^\\d+\\.\\d+\\.\\d+$")
})

test_that("version matches package version", {
  v <- version()
  pkg_version <- as.character(utils::packageVersion("htmltomarkdown"))
  expect_equal(v, pkg_version)
})
