defmodule HtmlToMarkdown do
  @moduledoc """
  High-level Elixir interface for the Rust html-to-markdown engine.
  """

  alias HtmlToMarkdown.{
    Error,
    Native,
    Options
  }

  @type options_input :: Options.t() | map() | keyword() | nil

  @doc """
  Convert HTML to Markdown, returning a `ConversionResult` map with:
  - `:content` - the converted Markdown string (or nil in extraction-only mode)
  - `:metadata` - extracted HTML metadata (document, headers, links, images, structured_data)
  - `:tables` - list of extracted tables with `:grid` (rows, cols, cells) and `:markdown`
  - `:warnings` - list of processing warnings with `:message` and `:kind`

  Returns `{:ok, result}` or `{:error, reason}`.
  """
  @spec convert(String.t(), options_input()) :: {:ok, map()} | {:error, term()}
  def convert(html, options \\ nil) when is_binary(html) do
    options_map = normalize_options(options) || %{}

    Native.convert(html, options_map)
  end

  @doc """
  Bang variant of `convert/2`. Raises on failure.
  """
  @spec convert!(String.t(), options_input()) :: map()
  def convert!(html, options \\ nil) do
    case convert(html, options) do
      {:ok, result} -> result
      {:error, reason} -> raise Error, message: inspect(reason)
    end
  end

  defp normalize_options(nil), do: nil

  defp normalize_options(options) do
    options
    |> Options.new()
    |> Options.to_map()
  end
end
