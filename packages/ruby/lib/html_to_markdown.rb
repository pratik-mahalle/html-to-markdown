# frozen_string_literal: true

require_relative 'html_to_markdown/version'
require 'html_to_markdown_rb'

module HtmlToMarkdown
  autoload :CLI, 'html_to_markdown/cli'
  autoload :CLIProxy, 'html_to_markdown/cli_proxy'

  class << self
    alias native_convert convert
  end

  module_function

  # Convert HTML to Markdown, returning a Hash with:
  #   - :content [String, nil] the converted Markdown output
  #   - :document [nil] document structure (not yet exposed)
  #   - :metadata [Hash, nil] extracted HTML metadata
  #   - :tables [Array<Hash>] extracted tables with :grid and :markdown
  #   - :images [Array<Hash>] extracted inline images
  #   - :warnings [Array<Hash>] processing warnings
  #
  # @param html [String] HTML string to convert
  # @param options [Hash, nil] optional conversion options
  # @return [Hash] conversion result
  def convert(html, options = nil)
    native_convert(html.to_s, options)
  end
end
