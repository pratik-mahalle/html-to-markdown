require 'html_to_markdown'
require 'json'

def load_fixtures(filename)
  fixture_path = File.join(__dir__, '../fixtures', filename)
  JSON.parse(File.read(fixture_path))
end

RSpec.describe 'comprehensive html-to-markdown tests' do
  # Basic HTML conversion tests
  describe 'Basic HTML Conversion' do
    basic_fixtures = load_fixtures('basic-html.json')

    basic_fixtures.each do |test_case|
      it test_case['name'] do
        result = HtmlToMarkdown.convert(test_case['html'], test_case['options'] || {})
        expect(result.strip).to eq(test_case['expectedMarkdown'].strip)
      end
    end
  end

  # Conversion options tests
  describe 'Conversion Options' do
    it 'applies heading style option (atx)' do
      html = '<h1>Title</h1>'
      result = HtmlToMarkdown.convert(html, heading_style: :atx)

      expect(result).to match(/^# Title/)
    end

    it 'applies heading style option (atx_closed)' do
      html = '<h1>Title</h1>'
      result = HtmlToMarkdown.convert(html, heading_style: :atx_closed)

      expect(result).to match(/#.*#/)
    end

    it 'applies list indent width option' do
      html = '<ul><li>Item 1<ul><li>Nested</li></ul></li></ul>'
      result = HtmlToMarkdown.convert(html, list_indent_width: 4)

      expect(result).to be_a(String)
    end

    it 'applies code block style option' do
      html = '<pre><code>code here</code></pre>'
      result = HtmlToMarkdown.convert(html, code_block_style: :fenced)

      expect(result).to include('```')
    end

    it 'applies text wrapping option' do
      long_text = 'A' * 200
      html = "<p>#{long_text}</p>"
      result = HtmlToMarkdown.convert(html, wrap: true, wrap_width: 80)

      expect(result).to be_a(String)
    end
  end

  # Error handling tests
  describe 'Error Handling' do
    it 'handles empty HTML gracefully' do
      result = HtmlToMarkdown.convert('')
      expect(result).to eq('')
    end

    it 'handles malformed HTML gracefully' do
      html = '<p>Unclosed paragraph<div>Nested'
      result = HtmlToMarkdown.convert(html)

      expect(result).to be_a(String)
    end

    it 'handles very large HTML documents' do
      large_html = '<p>' + ('x' * 100_000) + '</p>'
      result = HtmlToMarkdown.convert(large_html)

      expect(result).to be_a(String)
    end

    it 'handles special characters in HTML' do
      html = '<p>&lt;script&gt;alert("xss")&lt;/script&gt;</p>'
      result = HtmlToMarkdown.convert(html)

      expect(result).to be_a(String)
      expect(result).not_to include('<script>')
    end

    it 'handles unicode content' do
      html = '<p>Hello 世界 🌍</p>'
      result = HtmlToMarkdown.convert(html)

      expect(result).to include('世界')
      expect(result).to include('🌍')
    end
  end

  # API type definitions validation
  describe 'API Type Definitions' do
    it 'HtmlToMarkdown module responds to convert' do
      expect(HtmlToMarkdown).to respond_to(:convert)
    end
  end
end
