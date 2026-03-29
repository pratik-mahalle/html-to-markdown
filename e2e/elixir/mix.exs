defmodule HtmlToMarkdownE2e.MixProject do
  use Mix.Project

  def project do
    [
      app: :html_to_markdown_e2e,
      version: "0.1.0",
      elixir: "~> 1.19",
      start_permanent: false,
      deps: deps()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:html_to_markdown, path: "../../packages/elixir"}
    ]
  end
end
