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

  # Metadata extraction tests
  describe 'Metadata Extraction Feature' do
    it 'converts HTML with metadata extraction enabled' do
      html = '<html><head><title>Test</title></head><body><p>Content</p></body></html>'
      markdown, metadata = HtmlToMarkdown.convert_with_metadata(html)

      expect(markdown).to be_a(String)
      expect(metadata).to be_a(Hash)
      expect(markdown).to include('Content')
    end

    it 'extracts document metadata' do
      html = '<html><head><title>My Page</title><meta name="description" content="A test page"></head><body><p>Content</p></body></html>'
      _, metadata = HtmlToMarkdown.convert_with_metadata(html)

      expect(metadata).to have_key(:document)
      expect(metadata[:document][:title]).to eq('My Page')
      expect(metadata[:document][:description]).to eq('A test page')
    end

    it 'extracts headers from content' do
      html = '<h1>Title</h1><h2>Subtitle</h2><p>Content</p>'
      _, metadata = HtmlToMarkdown.convert_with_metadata(html)

      expect(metadata).to have_key(:headers)
      expect(metadata[:headers]).to be_a(Array)
    end

    it 'extracts links from content' do
      html = '<p><a href="https://example.com">Link</a></p>'
      _, metadata = HtmlToMarkdown.convert_with_metadata(html)

      expect(metadata).to have_key(:links)
      expect(metadata[:links]).to be_a(Array)
    end

    it 'extracts images from content' do
      html = '<p><img src="https://example.com/image.png" alt="Example"></p>'
      _, metadata = HtmlToMarkdown.convert_with_metadata(html)

      expect(metadata).to have_key(:images)
      expect(metadata[:images]).to be_a(Array)
    end
  end

  # Inline images extraction tests
  describe 'Inline Images Feature' do
    it 'returns hash with markdown, inline_images, and warnings' do
      html = '<p><img src="data:image/png;base64,ZmFrZQ==" alt="Test"></p>'
      result = HtmlToMarkdown.convert_with_inline_images(html)

      expect(result).to be_a(Hash)
      expect(result).to have_key(:markdown)
      expect(result).to have_key(:inline_images)
      expect(result).to have_key(:warnings)
    end

    it 'extracts base64-encoded inline images' do
      html = '<p><img src="data:image/png;base64,ZmFrZQ==" alt="Inline image"></p>'
      result = HtmlToMarkdown.convert_with_inline_images(html)

      expect(result[:inline_images]).to be_a(Array)
      unless result[:inline_images].empty?
        expect(result[:inline_images].first).to have_key(:data)
        expect(result[:inline_images].first).to have_key(:description)
      end
    end

    it 'handles multiple inline images' do
      html = '<p><img src="data:image/png;base64,ZmFrZTA=" alt="Image 1"><img src="data:image/png;base64,ZmFrZTE=" alt="Image 2"></p>'
      result = HtmlToMarkdown.convert_with_inline_images(html)

      expect(result[:inline_images]).to be_a(Array)
    end
  end

  # Visitor pattern tests
  describe 'Visitor Pattern Feature' do
    it 'converts HTML with visitor callbacks' do
      html = '<p>Test</p>'
      visitor = double('visitor')
      allow(visitor).to receive(:visit_text).and_return({ type: :continue })
      allow(visitor).to receive(:visit_element_start).and_return({ type: :continue })
      allow(visitor).to receive(:visit_element_end).and_return({ type: :continue })

      result = HtmlToMarkdown.convert_with_visitor(html, nil, visitor)

      expect(result).to be_a(String)
      expect(result).to include('Test')
    end

    it 'supports custom element processing via visitor' do
      html = '<div class="custom">Custom content</div>'
      visitor = double('visitor')
      allow(visitor).to receive(:visit_element_start).and_return({ type: :continue })
      allow(visitor).to receive(:visit_element_end).and_return({ type: :continue })
      allow(visitor).to receive(:visit_text).and_return({ type: :continue })

      result = HtmlToMarkdown.convert_with_visitor(html, nil, visitor)

      expect(result).to be_a(String)
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

  # RBS type definitions validation
  describe 'RBS Type Definitions' do
    it 'HtmlToMarkdown module responds to convert' do
      expect(HtmlToMarkdown).to respond_to(:convert)
    end

    it 'HtmlToMarkdown module responds to convert_with_metadata' do
      expect(HtmlToMarkdown).to respond_to(:convert_with_metadata)
    end

    it 'HtmlToMarkdown module responds to convert_with_inline_images' do
      expect(HtmlToMarkdown).to respond_to(:convert_with_inline_images)
    end

    it 'HtmlToMarkdown module responds to convert_with_visitor' do
      expect(HtmlToMarkdown).to respond_to(:convert_with_visitor)
    end

    it 'HtmlToMarkdown::Options class exists' do
      expect(defined?(HtmlToMarkdown::Options)).to be_truthy
    end
  end
end
