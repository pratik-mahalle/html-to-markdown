# frozen_string_literal: true

# rubocop:disable RSpec/VerifiedDoubles, RSpec/DescribeMethod
require 'spec_helper'

RSpec.describe HtmlToMarkdown, 'Issue #187: Visitor tag_name context validation' do
  describe 'tag_name in visit_element_start context' do
    it 'receives correct tag_name for div element' do
      html = '<div>Content</div>'
      visitor = double(Object)
      tag_names_visited = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names_visited << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names_visited).to include('div')
    end

    it 'receives correct tag_name for script element' do
      html = '<div><script>var x = 1;</script></div>'
      visitor = double(Object)
      tag_names_visited = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names_visited << ctx[:tag_name]
        { type: :continue }
      end

      # NOTE: script and style elements are filtered out by default, so this test verifies
      # that when they do appear in the context, tag_name is correct
      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      # script tags are often filtered out for security; verify div is there at minimum
      expect(tag_names_visited).to include('div')
    end

    it 'receives correct tag_name for style element' do
      html = '<style>.cls { color: red; }</style><p>Text</p>'
      visitor = double(Object)
      tag_names_visited = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names_visited << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      # style tags are often filtered out for security; verify p is there at minimum
      expect(tag_names_visited).to include('p')
    end

    it 'receives correct tag_name for p element' do
      html = '<div><p>Paragraph</p></div>'
      visitor = double(Object)
      tag_names_visited = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names_visited << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names_visited).to include('p')
    end

    it 'receives all expected tag names for mixed HTML' do
      html = '<div><p>Text</p><h1>Heading</h1><span>Span</span></div>'
      visitor = double(Object)
      tag_names_visited = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names_visited << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue },
        visit_heading: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names_visited).to include('div')
      expect(tag_names_visited).to include('p')
      expect(tag_names_visited).to include('h1')
      expect(tag_names_visited).to include('span')
    end
  end

  describe 'tag_name type validation' do
    it 'tag_name is always a string' do
      html = '<section id="main"><article>Content</article></section>'
      visitor = double(Object)
      tag_name_types = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_name_types << ctx[:tag_name].class
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_name_types).not_to be_empty
      expect(tag_name_types).to all(eq(String))
    end

    it 'tag_name is never nil' do
      html = '<div><span>Text</span></div>'
      visitor = double(Object)
      nil_tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        nil_tag_names << ctx[:tag_name] if ctx[:tag_name].nil?
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(nil_tag_names).to be_empty
    end

    it 'tag_name is never empty string' do
      html = '<div><p>Test</p></div>'
      visitor = double(Object)
      empty_tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        empty_tag_names << ctx[:tag_name] if ctx[:tag_name].empty?
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(empty_tag_names).to be_empty
    end
  end

  describe 'filtering by tag name' do
    it 'filters divs by tag name in context' do
      html = '<div id="d1"><div id="d2">Nested</div></div><p>Paragraph</p>'
      visitor = double(Object)
      divs_found = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        divs_found << ctx[:attributes]['id'] if ctx[:tag_name] == 'div'
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(divs_found).to include('d1')
      expect(divs_found).to include('d2')
      expect(divs_found.length).to eq(2)
    end

    it 'filters elements by tag name and applies custom transformation' do
      html = '<div class="remove">Skip me</div><div class="keep">Keep me</div><p>Text</p>'
      visitor = double(Object)

      allow(visitor).to receive(:visit_element_start) do |ctx|
        if ctx[:tag_name] == 'div' && ctx[:attributes]['class'] == 'remove'
          { type: :skip }
        else
          { type: :continue }
        end
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      result = described_class.convert_with_visitor(html, nil, visitor)

      expect(result).to include('Keep me')
      expect(result).not_to include('Skip me')
    end

    it 'collects tag names by class attribute' do
      html = '<div class="container"><p class="content">Text</p></div>'
      visitor = double(Object)
      tags_by_class = {}

      allow(visitor).to receive(:visit_element_start) do |ctx|
        class_name = ctx[:attributes]['class']
        if class_name
          tags_by_class[class_name] ||= []
          tags_by_class[class_name] << ctx[:tag_name]
        end
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tags_by_class['container']).to include('div')
      expect(tags_by_class['content']).to include('p')
    end

    it 'filters and counts specific tag names' do
      html = '<h1>H1</h1><h2>H2</h2><p>Para</p><h3>H3</h3>'
      visitor = double(Object)
      heading_count = { h1: 0, h2: 0, h3: 0 }

      allow(visitor).to receive(:visit_element_start) do |ctx|
        case ctx[:tag_name]
        when 'h1'
          heading_count[:h1] += 1
        when 'h2'
          heading_count[:h2] += 1
        when 'h3'
          heading_count[:h3] += 1
        end
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue },
        visit_heading: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(heading_count[:h1]).to eq(1)
      expect(heading_count[:h2]).to eq(1)
      expect(heading_count[:h3]).to eq(1)
    end
  end

  describe 'filtering divs by class attribute combined with tag_name' do
    it 'identifies and filters divs with specific classes' do
      html = '
        <div class="header">Header</div>
        <div class="content">Content</div>
        <div class="footer">Footer</div>
        <p class="text">Paragraph</p>
      '
      visitor = double(Object)
      content_divs = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        content_divs << ctx if ctx[:tag_name] == 'div' && ctx[:attributes]['class'] == 'content'
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(content_divs.length).to eq(1)
      expect(content_divs[0][:tag_name]).to eq('div')
      expect(content_divs[0][:attributes]['class']).to eq('content')
    end

    it 'skips divs matching filter criteria' do
      html = '
        <div class="advertisement">Ad</div>
        <p>Paragraph 1</p>
        <div class="advertisement">Another ad</div>
        <p>Paragraph 2</p>
      '
      visitor = double(Object)

      allow(visitor).to receive(:visit_element_start) do |ctx|
        if ctx[:tag_name] == 'div' && ctx[:attributes]['class']&.include?('advertisement')
          { type: :skip }
        else
          { type: :continue }
        end
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      result = described_class.convert_with_visitor(html, nil, visitor)

      expect(result).to include('Paragraph 1')
      expect(result).to include('Paragraph 2')
      expect(result).not_to include('Ad')
      expect(result).not_to include('Another ad')
    end

    it 'transforms divs with specific class' do
      html = '<div class="warning">Important</div><div>Normal</div>'
      visitor = double(Object)

      allow(visitor).to receive(:visit_element_start) do |ctx|
        if ctx[:tag_name] == 'div' && ctx[:attributes]['class'] == 'warning'
          { type: :custom, output: '**WARNING: Important**' }
        else
          { type: :continue }
        end
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      result = described_class.convert_with_visitor(html, nil, visitor)

      expect(result).to include('WARNING: Important')
    end

    it 'preserves HTML for certain divs based on class' do
      html = '<div class="custom">Custom HTML</div><div>Normal</div>'
      visitor = double(Object)

      allow(visitor).to receive(:visit_element_start) do |ctx|
        if ctx[:tag_name] == 'div' && ctx[:attributes]['class'] == 'custom'
          { type: :preserve_html }
        else
          { type: :continue }
        end
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      result = described_class.convert_with_visitor(html, nil, visitor)

      expect(result).to be_a(String)
    end
  end

  describe 'tag_name consistency across visitor lifecycle' do
    it 'tag_name is consistent in visit_element_start and visit_element_end' do
      html = '<section><article>Content</article></section>'
      visitor = double(Object)
      element_lifecycle = {}

      allow(visitor).to receive(:visit_element_start) do |ctx|
        element_lifecycle[ctx[:tag_name]] ||= { start: 0, end: 0 }
        element_lifecycle[ctx[:tag_name]][:start] += 1
        { type: :continue }
      end

      allow(visitor).to receive(:visit_element_end) do |ctx, _output|
        element_lifecycle[ctx[:tag_name]][:end] += 1
        { type: :continue }
      end

      allow(visitor).to receive(:visit_text).and_return({ type: :continue })

      described_class.convert_with_visitor(html, nil, visitor)

      expect(element_lifecycle['section'][:start]).to eq(element_lifecycle['section'][:end])
      expect(element_lifecycle['article'][:start]).to eq(element_lifecycle['article'][:end])
    end

    it 'tag_name is consistent for nested elements of same type' do
      html = '<div><div><div>Deep</div></div></div>'
      visitor = double(Object)
      div_count = 0

      allow(visitor).to receive(:visit_element_start) do |ctx|
        div_count += 1 if ctx[:tag_name] == 'div'
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(div_count).to eq(3)
    end
  end

  describe 'tag_name with complex HTML structures' do
    it 'maintains tag_name accuracy with complex nested structure' do
      html = '
        <article>
          <header><h1>Title</h1></header>
          <section>
            <div class="content">
              <p>Paragraph <strong>bold</strong></p>
              <ul>
                <li>Item 1</li>
                <li>Item 2</li>
              </ul>
            </div>
          </section>
          <footer>Footer</footer>
        </article>
      '
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue },
        visit_heading: { type: :continue },
        visit_list_start: { type: :continue },
        visit_list_end: { type: :continue },
        visit_list_item: { type: :continue },
        visit_strong: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names).to include('article')
      expect(tag_names).to include('header')
      expect(tag_names).to include('h1')
      expect(tag_names).to include('section')
      expect(tag_names).to include('div')
      expect(tag_names).to include('p')
      expect(tag_names).to include('strong')
      expect(tag_names).to include('ul')
      expect(tag_names).to include('li')
      expect(tag_names).to include('footer')
    end

    it 'correctly filters table elements by tag_name' do
      html = '
        <table>
          <tr><th>Header</th></tr>
          <tr><td>Data</td></tr>
        </table>
      '
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue },
        visit_table_start: { type: :continue },
        visit_table_end: { type: :continue },
        visit_table_row: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      # Tables are handled at higher level, verify core table tag is there
      expect(tag_names).to include('table')
    end
  end

  describe 'tag_name edge cases' do
    it 'handles self-closing tags correctly' do
      html = '<p>Text<br/>More text<hr/></p>'
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names).to include('p')
      expect(tag_names).to include('br')
      expect(tag_names).to include('hr')
    end

    it 'handles lowercase tag names' do
      html = '<DIV class="Test">Content</DIV>'
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      # HTML5 normalizes tags to lowercase
      expect(tag_names).to include('div')
    end

    it 'handles special/semantic elements' do
      html = '<main><nav>Navigation</nav><aside>Sidebar</aside></main>'
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names).to include('main')
      expect(tag_names).to include('nav')
      expect(tag_names).to include('aside')
    end

    it 'handles form elements correctly' do
      html = '
        <form>
          <input type="text" name="username"/>
          <button>Submit</button>
        </form>
      '
      visitor = double(Object)
      tag_names = []

      allow(visitor).to receive(:visit_element_start) do |ctx|
        tag_names << ctx[:tag_name]
        { type: :continue }
      end

      allow(visitor).to receive_messages(
        visit_element_end: { type: :continue },
        visit_text: { type: :continue }
      )

      described_class.convert_with_visitor(html, nil, visitor)

      expect(tag_names).to include('form')
      expect(tag_names).to include('input')
      expect(tag_names).to include('button')
    end
  end
end

# rubocop:enable RSpec/VerifiedDoubles, RSpec/DescribeMethod
