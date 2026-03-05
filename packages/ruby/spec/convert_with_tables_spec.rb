# frozen_string_literal: true

require 'spec_helper'

RSpec.describe HtmlToMarkdown do
  describe '.convert_with_tables' do
    it 'returns a hash with content, metadata, and tables keys' do
      html = '<table><tr><td>Cell</td></tr></table>'
      result = described_class.convert_with_tables(html)

      expect(result).to be_a(Hash)
      expect(result).to include(:content, :metadata, :tables)
    end

    context 'with a basic table with header' do
      let(:html) do
        <<~HTML
          <table>
            <thead>
              <tr><th>Name</th><th>Age</th></tr>
            </thead>
            <tbody>
              <tr><td>Alice</td><td>30</td></tr>
            </tbody>
          </table>
        HTML
      end

      it 'extracts exactly one table' do
        result = described_class.convert_with_tables(html)

        expect(result[:tables].length).to eq(1)
      end

      it 'extracts cells as rows of columns' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:cells]).to be_an(Array)
        expect(table[:cells].length).to eq(2)
        expect(table[:cells][0]).to eq(%w[Name Age])
        expect(table[:cells][1]).to eq(%w[Alice 30])
      end

      it 'provides markdown representation' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:markdown]).to be_a(String)
        expect(table[:markdown]).to include('Name')
        expect(table[:markdown]).to include('Alice')
      end

      it 'marks header rows correctly' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:is_header_row]).to be_an(Array)
        expect(table[:is_header_row].length).to eq(2)
        expect(table[:is_header_row][0]).to be true
        expect(table[:is_header_row][1]).to be false
      end

      it 'includes converted markdown content' do
        result = described_class.convert_with_tables(html)

        expect(result[:content]).to be_a(String)
        expect(result[:content]).not_to be_empty
      end
    end

    context 'with empty HTML' do
      it 'returns empty tables array' do
        result = described_class.convert_with_tables('')

        expect(result[:tables]).to eq([])
        expect(result[:content]).to be_a(String)
      end
    end

    context 'with HTML containing no tables' do
      it 'returns empty tables array' do
        html = '<p>No tables here</p>'
        result = described_class.convert_with_tables(html)

        expect(result[:tables]).to eq([])
        expect(result[:content]).to include('No tables here')
      end
    end

    context 'with multiple tables' do
      let(:html) do
        <<~HTML
          <table>
            <tr><th>A</th></tr>
            <tr><td>1</td></tr>
          </table>
          <p>Some text between tables</p>
          <table>
            <tr><th>B</th><th>C</th></tr>
            <tr><td>2</td><td>3</td></tr>
            <tr><td>4</td><td>5</td></tr>
          </table>
        HTML
      end

      it 'extracts all tables' do
        result = described_class.convert_with_tables(html)

        expect(result[:tables].length).to eq(2)
      end

      it 'preserves table order' do
        result = described_class.convert_with_tables(html)

        first_table = result[:tables][0]
        second_table = result[:tables][1]

        expect(first_table[:cells][0]).to eq(['A'])
        expect(second_table[:cells][0]).to eq(%w[B C])
      end

      it 'extracts correct row counts per table' do
        result = described_class.convert_with_tables(html)

        expect(result[:tables][0][:cells].length).to eq(2)
        expect(result[:tables][1][:cells].length).to eq(3)
      end
    end

    context 'with special characters in cells' do
      let(:html) do
        <<~HTML
          <table>
            <tr><th>Key</th><th>Value</th></tr>
            <tr><td>Brackets &lt;&gt;</td><td>Ampersand &amp;</td></tr>
            <tr><td>Quotes "double"</td><td>Quotes 'single'</td></tr>
            <tr><td>Unicode: cafe\u0301</td><td>Emoji: test</td></tr>
          </table>
        HTML
      end

      it 'handles HTML entities in cells' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:cells][1][0]).to include('<>')
        expect(table[:cells][1][1]).to include('&')
      end

      it 'handles quotes in cells' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:cells][2][0]).to include('"double"')
        expect(table[:cells][2][1]).to include("'single'")
      end

      it 'handles unicode in cells' do
        result = described_class.convert_with_tables(html)
        table = result[:tables][0]

        expect(table[:cells][3][0]).to be_a(String)
      end
    end

    context 'with conversion options' do
      it 'accepts options hash' do
        html = '<table><tr><th>Header</th></tr><tr><td>Data</td></tr></table>'
        result = described_class.convert_with_tables(html, { heading_style: :atx })

        expect(result).to be_a(Hash)
        expect(result[:tables].length).to eq(1)
      end

      it 'accepts nil options' do
        html = '<table><tr><td>Data</td></tr></table>'
        result = described_class.convert_with_tables(html, nil, nil)

        expect(result).to be_a(Hash)
        expect(result[:tables].length).to eq(1)
      end
    end

    context 'with metadata config' do
      it 'includes metadata when configured' do
        html = '<html><head><title>Test</title></head><body><table><tr><td>Data</td></tr></table></body></html>'
        result = described_class.convert_with_tables(html, nil, { extract_headers: true })

        expect(result[:metadata]).to be_a(Hash).or(be_nil)
      end
    end
  end
end
