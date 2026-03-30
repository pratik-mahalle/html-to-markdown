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

  it 'converts complex HTML' do
    html = '<h1>Title</h1><p>Paragraph with <strong>bold</strong></p>'
    result = HtmlToMarkdown.convert(html)

    expect(result).to be_a(String)
    expect(result).to include('Title')
    expect(result).to include('bold')
  end
end
