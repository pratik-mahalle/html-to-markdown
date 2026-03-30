# frozen_string_literal: true

require 'spec_helper'

RSpec.describe HtmlToMarkdown do
  describe '.convert' do
    it 'converts simple headings' do
      expect(described_class.convert('<h1>Hello</h1>')).to eq("# Hello\n")
    end

    it 'accepts options hash' do
      result = described_class.convert(
        '<h1>Hello</h1>',
        heading_style: :atx_closed,
        default_title: true
      )
      expect(result).to include('Hello')
    end

    it 'converts inline images' do
      html = '<p><img src="data:image/png;base64,ZmFrZQ==" alt="fake"></p>'
      result = described_class.convert(html)
      expect(result).to be_a(String)
    end
  end

  describe 'panic handling' do
    context 'when a Rust panic would occur' do
      it 'catches panics in convert method' do
        malformed_html = "#{'<' * 100_000}div#{'>' * 100_000}"

        begin
          result = described_class.convert(malformed_html)
          expect(result).to be_a(String)
        rescue RuntimeError => e
          expect(e.message).to match(/html-to-markdown panic during conversion/)
        end
      end

      it 'catches panics in convert with options' do
        malformed_html = "#{'<' * 100_000}div#{'>' * 100_000}"

        begin
          result = described_class.convert(malformed_html, heading_style: :atx)
          expect(result).to be_a(String)
        rescue RuntimeError => e
          expect(e.message).to match(/html-to-markdown panic during conversion/)
        end
      end
    end
  end
end
