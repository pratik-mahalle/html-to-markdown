defmodule HtmlToMarkdown.Native do
  @moduledoc false

  @features System.get_env("HTML_TO_MARKDOWN_CARGO_FEATURES", "")
            |> String.split(",", trim: true)

  use Rustler,
    otp_app: :html_to_markdown,
    crate: "html_to_markdown_elixir",
    path: "native/html_to_markdown_elixir",
    mode: (Mix.env() == :prod && :release) || :debug,
    features: @features

  def convert(_html, _options), do: :erlang.nif_error(:nif_not_loaded)
end
