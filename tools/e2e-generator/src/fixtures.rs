//! Fixture data structures and loader for html-to-markdown e2e tests.

use anyhow::{Context, Result, bail};
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use walkdir::WalkDir;

/// A single test fixture describing HTML input, options, and assertions.
#[derive(Debug, Deserialize)]
pub struct Fixture {
    /// Unique identifier for this fixture (used as test function name).
    pub id: String,

    /// Optional category override (defaults to parent directory name).
    pub category: Option<String>,

    /// Human-readable description of what this fixture tests.
    pub description: String,

    /// Optional tags for filtering.
    pub tags: Option<Vec<String>>,

    /// Inline HTML input string.
    pub html: Option<String>,

    /// Path to an HTML file in test_documents/ (relative to repo root).
    pub html_file: Option<String>,

    /// Conversion options to apply (map of option name -> value).
    pub options: Option<BTreeMap<String, serde_json::Value>>,

    /// Assertions to verify on the conversion result.
    #[serde(default)]
    pub assertions: Assertions,

    /// Skip directive to exclude this fixture from certain languages.
    pub skip: Option<SkipDirective>,

    /// Source file path (populated during loading, not from JSON).
    #[serde(skip)]
    pub source: Utf8PathBuf,
}

/// Assertions to run against a [`crate::ConversionResult`].
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Assertions {
    // ---- Content assertions ----
    /// The content must exactly equal this string (after trimming).
    pub content_equals: Option<String>,

    /// The content must contain all of these substrings.
    pub content_contains_all: Option<Vec<String>>,

    /// The content must contain at least one of these substrings.
    pub content_contains_any: Option<Vec<String>>,

    /// The content must not contain any of these substrings.
    pub content_not_contains: Option<Vec<String>>,

    /// The content must be non-empty (when `true`).
    pub content_not_empty: Option<bool>,

    /// The content must be `None` or empty (when `true`).
    pub content_is_none: Option<bool>,

    /// The content must be at least this many characters long.
    pub min_content_length: Option<usize>,

    /// The content must be at most this many characters long.
    pub max_content_length: Option<usize>,

    /// The content must start with this string.
    pub content_starts_with: Option<String>,

    /// The content must end with this string.
    pub content_ends_with: Option<String>,

    /// The content must match this regex pattern.
    pub content_matches_regex: Option<String>,

    // ---- Structure assertions ----
    /// The result must include a populated document structure.
    pub has_document_structure: Option<bool>,

    /// The document must have at least this many nodes.
    pub document_node_count_min: Option<usize>,

    /// The document must include nodes of these types.
    pub document_node_types_include: Option<Vec<String>>,

    // ---- Metadata assertions ----
    /// The metadata title must equal this string.
    pub metadata_title: Option<String>,

    /// The metadata must include extracted links (when `true`).
    pub metadata_has_links: Option<bool>,

    /// The metadata must have at least this many links.
    pub metadata_link_count_min: Option<usize>,

    /// The metadata must include extracted headers (when `true`).
    pub metadata_has_headers: Option<bool>,

    /// The metadata must have at least this many headers.
    pub metadata_header_count_min: Option<usize>,

    // ---- Table assertions ----
    /// The result must include at least this many tables.
    pub table_count_min: Option<usize>,

    /// At least one table cell must contain this string.
    pub table_contains_cell: Option<String>,

    // ---- Image assertions ----
    /// The result must include at least this many inline images.
    pub image_count_min: Option<usize>,

    // ---- Warning assertions ----
    /// The warnings list must be empty (when `true`).
    pub warnings_empty: Option<bool>,

    // ---- Error assertions ----
    /// The conversion must return an error (when `true`).
    pub expect_error: Option<bool>,

    /// The error message must contain this string.
    pub error_contains: Option<String>,
}

/// Directive to skip this fixture for certain languages.
#[derive(Debug, Deserialize)]
pub struct SkipDirective {
    /// Language names to skip (e.g. `["go", "java"]`). `None` means skip all.
    pub languages: Option<Vec<String>>,

    /// Human-readable reason for skipping.
    pub reason: Option<String>,
}

impl Fixture {
    /// Returns the resolved category: explicit `category` field or parent directory name.
    pub fn resolved_category(&self) -> &str {
        if let Some(cat) = &self.category {
            cat.as_str()
        } else {
            self.source
                .parent()
                .and_then(|p| p.file_name())
                .unwrap_or("uncategorized")
        }
    }

    /// Returns the HTML content for this fixture.
    ///
    /// Prefers `html` over `html_file`. Returns an error if neither is set.
    pub fn html_content(&self) -> Result<&str> {
        if let Some(html) = &self.html {
            return Ok(html.as_str());
        }
        bail!(
            "fixture '{}' has no 'html' field (html_file not supported inline)",
            self.id
        );
    }

    /// Returns `true` if this fixture should be skipped for the given language.
    pub fn is_skipped_for(&self, lang: &str) -> bool {
        match &self.skip {
            None => false,
            Some(skip) => match &skip.languages {
                None => true, // skip all languages
                Some(langs) => langs.iter().any(|l| l.eq_ignore_ascii_case(lang)),
            },
        }
    }
}

/// Load all fixtures from a directory, walking recursively.
///
/// - Files named `schema.json` or starting with `_` are skipped.
/// - Each file may contain a single fixture object or an array of fixture objects.
/// - Category defaults to the parent directory name if not specified in the fixture.
/// - Duplicate IDs cause an error.
/// - Results are sorted by (category, id).
pub fn load_fixtures(dir: &Utf8Path) -> Result<Vec<Fixture>> {
    let mut fixtures: Vec<Fixture> = Vec::new();
    let mut seen_ids: BTreeSet<String> = BTreeSet::new();

    for entry in WalkDir::new(dir).sort_by_file_name().into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "json" {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if file_name == "schema.json" || file_name.starts_with('_') {
            continue;
        }

        let utf8_path = Utf8Path::from_path(path).with_context(|| format!("Non-UTF-8 path: {}", path.display()))?;

        let raw = std::fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

        let loaded =
            parse_fixture_file(&raw, utf8_path).with_context(|| format!("Failed to parse {}", path.display()))?;

        for mut fixture in loaded {
            if fixture.source.as_str().is_empty() {
                fixture.source = utf8_path.to_owned();
            }

            if seen_ids.contains(&fixture.id) {
                bail!("Duplicate fixture id '{}' found in {}", fixture.id, utf8_path);
            }
            seen_ids.insert(fixture.id.clone());
            fixtures.push(fixture);
        }
    }

    // Sort by (category, id) for stable output.
    fixtures.sort_by(|a, b| {
        a.resolved_category()
            .cmp(b.resolved_category())
            .then_with(|| a.id.cmp(&b.id))
    });

    Ok(fixtures)
}

/// Parse a fixture file that may contain either a single JSON object or a JSON array.
fn parse_fixture_file(raw: &str, source: &Utf8Path) -> Result<Vec<Fixture>> {
    let value: serde_json::Value = serde_json::from_str(raw)?;

    let items = match value {
        serde_json::Value::Array(arr) => arr,
        obj @ serde_json::Value::Object(_) => vec![obj],
        other => bail!("Expected JSON object or array, got {}", other),
    };

    let mut fixtures = Vec::with_capacity(items.len());
    for (i, item) in items.into_iter().enumerate() {
        let mut fixture: Fixture =
            serde_json::from_value(item).with_context(|| format!("Failed to deserialize fixture at index {i}"))?;
        fixture.source = source.to_owned();
        fixtures.push(fixture);
    }

    Ok(fixtures)
}
