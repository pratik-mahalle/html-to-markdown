//! Table extraction conversion functions for Ruby bindings.

use html_to_markdown_rs::{ConversionWithTables, TableData};
use magnus::prelude::*;
use magnus::{Error, Ruby, Value};

#[cfg(feature = "metadata")]
use super::metadata::extended_metadata_to_ruby;

fn table_data_to_ruby(ruby: &Ruby, table: TableData) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    // cells: Array[Array[String]]
    let cells_array = ruby.ary_new();
    for row in table.cells {
        let row_array = ruby.ary_new();
        for cell in row {
            row_array.push(cell)?;
        }
        cells_array.push(row_array)?;
    }
    hash.aset(ruby.intern("cells"), cells_array)?;

    // markdown: String
    hash.aset(ruby.intern("markdown"), table.markdown)?;

    // is_header_row: Array[bool]
    let header_array = ruby.ary_new();
    for is_header in table.is_header_row {
        header_array.push(is_header)?;
    }
    hash.aset(ruby.intern("is_header_row"), header_array)?;

    Ok(hash.as_value())
}

/// Convert a `ConversionWithTables` result to a Ruby Hash.
///
/// Returns a Hash with keys `:content`, `:metadata`, `:tables`.
pub fn tables_result_to_ruby(ruby: &Ruby, result: ConversionWithTables) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    // content: String
    hash.aset(ruby.intern("content"), result.content)?;

    // metadata: Hash or nil
    #[cfg(feature = "metadata")]
    {
        match result.metadata {
            Some(metadata) => {
                hash.aset(ruby.intern("metadata"), extended_metadata_to_ruby(ruby, metadata)?)?;
            }
            None => {
                hash.aset(ruby.intern("metadata"), ruby.qnil())?;
            }
        }
    }
    #[cfg(not(feature = "metadata"))]
    {
        hash.aset(ruby.intern("metadata"), ruby.qnil())?;
    }

    // tables: Array[Hash]
    let tables_array = ruby.ary_new();
    for table in result.tables {
        tables_array.push(table_data_to_ruby(ruby, table)?)?;
    }
    hash.aset(ruby.intern("tables"), tables_array)?;

    Ok(hash.as_value())
}
