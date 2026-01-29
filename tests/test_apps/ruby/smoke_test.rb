require 'html_to_markdown'

RSpec.describe 'html-to-markdown smoke tests' do
  it 'can load the gem' do
    expect(defined?(HtmlToMarkdown)).to be_truthy
  end

  it 'gem is installed from RubyGems (not local path)' do
    # Verify we're using the published gem by checking that it's in the gems directory
    # and not a local path dependency
    gem_spec = Gem::Specification.find_by_name('html-to-markdown')
    gem_location = gem_spec.gem_dir

    # Should not be a path dependency (would contain /packages/ruby)
    expect(gem_location).not_to include('packages/ruby')
    # Should be in the bundle gems directory
    expect(gem_location).to include('bundle') or expect(gem_location).to include('gems')
  end

  it 'converts basic HTML' do
    html = '<p>Hello World</p>'
    result = HtmlToMarkdown.convert(html)
    expect(result).to include('Hello World')
  end

  it 'handles options' do
    html = '<h1>Title</h1>'
    result = HtmlToMarkdown.convert(html)
    expect(result).to start_with('#')
  end

  it 'handles empty input' do
    result = HtmlToMarkdown.convert('')
    expect(result).to eq('')
  end

  it 'supports convert_with_metadata' do
    html = '<html><head><title>Test</title></head><body><p>Content</p></body></html>'
    markdown, metadata = HtmlToMarkdown.convert_with_metadata(html)

    expect(markdown).to be_a(String)
    expect(metadata).to be_a(Hash)
  end

  it 'supports convert_with_inline_images' do
    html = '<p><img src="data:image/png;base64,ZmFrZQ==" alt="Test"></p>'
    result = HtmlToMarkdown.convert_with_inline_images(html)

    expect(result).to be_a(Hash)
    expect(result).to have_key(:markdown)
    expect(result).to have_key(:inline_images)
  end

  it 'supports convert_with_visitor' do
    html = '<p>Test</p>'
    visitor = double('visitor')
    allow(visitor).to receive(:visit_text).and_return({ type: :continue })

    result = HtmlToMarkdown.convert_with_visitor(html, nil, visitor)
    expect(result).to be_a(String)
  end
end
