# Visitor Pattern - R

Customize HTML to Markdown conversion with visitor callbacks.

## Basic Visitor Usage

Use `convert_with_visitor()` for conversion with a visitor parameter:

```r
library(htmltomarkdown)

html <- "<p>Visit <a href='https://example.com'>our site</a> for more!</p>"

# Currently performs standard conversion
# The visitor parameter is reserved for future callback support
markdown <- convert_with_visitor(html)
cat(markdown)
#> Visit [our site](https://example.com) for more!
```

## With Options

Combine visitor conversion with options:

```r
opts <- conversion_options(wrap = TRUE, wrap_width = 80L)
markdown <- convert_with_visitor(html, visitor = NULL, options = opts)
```

## Future Callback Support

The visitor pattern will support R callback functions for customizing
how specific HTML elements are converted to Markdown. This mirrors
the visitor API available in Python, Elixir, Ruby, and PHP bindings.
