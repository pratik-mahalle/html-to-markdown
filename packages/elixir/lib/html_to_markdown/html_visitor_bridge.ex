defmodule HtmlToMarkdownHtmlVisitorBridge do
  @moduledoc """
  GenServer bridge for HtmlVisitor implementation in html_to_markdown.

  Handles incoming trait method calls from Rust and dispatches them to an implementation module.
  """

  use GenServer

  require Logger

  @doc """
  Start a GenServer linked to the current process.

  impl_module should be a module that implements the HtmlVisitor trait methods.
  """
  def start_link(impl_module) do
    GenServer.start_link(__MODULE__, impl_module, name: __MODULE__)
  end

  @impl GenServer
  def init(impl_module) do
    {:ok, impl_module}
  end

  @doc """
  Handle an incoming trait call message.

  Message format: {:trait_call, method_atom, args_json, reply_id}
  """
  @impl GenServer
  def handle_info({:trait_call, method, args_json, reply_id}, impl_module) do
    try do
      args = Jason.decode!(args_json)

      # Dispatch to the implementation module
      result = apply(impl_module, String.to_atom(method), args)

      # Send result back to Rust
      HtmlToMarkdown.Native.complete_trait_call(reply_id, Jason.encode!(result))
    rescue
      e ->
        Logger.error("Error calling {impl_module}.{method}: {Exception.message(e)}")
        HtmlToMarkdown.Native.fail_trait_call(reply_id, Exception.message(e))
    end

    {:noreply, impl_module}
  end

  @doc """
  Register an implementation module, starting a GenServer to handle trait calls.
  """
  def register(impl_module) do
    {:ok, _pid} = start_link(impl_module)
    HtmlToMarkdown.Native.register_html_visitor(self(), Atom.to_string(impl_module))
  end
end
