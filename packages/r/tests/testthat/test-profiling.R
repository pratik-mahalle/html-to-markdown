test_that("start_profiling without feature returns error", {
  # Profiling is disabled by default (no 'profiling' feature)
  expect_error(start_profiling("/tmp/test.svg"))
})

test_that("stop_profiling without active session returns error", {
  expect_error(stop_profiling())
})
