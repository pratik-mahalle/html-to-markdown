//! E2e test generator for html-to-markdown.
//!
//! Reads test fixtures from a directory and generates language-specific test files.
//!
//! ## Commands
//!
//! ```text
//! html-to-markdown-e2e-generator generate --lang <LANG> [--fixtures fixtures/] [--output e2e/]
//! html-to-markdown-e2e-generator list [--fixtures fixtures/]
//! ```

mod c;
mod csharp;
mod elixir;
mod fixtures;
mod go;
mod java;
mod php;
mod python;
mod r;
mod ruby;
mod rust;
mod typescript;
mod wasm;

use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Parser, Subcommand, ValueEnum};
use fixtures::load_fixtures;

/// CLI for generating html-to-markdown e2e tests from fixture files.
#[derive(Parser)]
#[command(name = "html-to-markdown-e2e-generator")]
#[command(about = "Generate e2e tests from fixtures", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate e2e tests for one or all supported languages.
    Generate {
        /// Target language(s) to generate tests for.
        #[arg(short, long, value_enum, default_value = "all")]
        lang: Language,

        /// Directory containing fixture JSON files.
        #[arg(short, long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,

        /// Output directory for generated tests.
        #[arg(short, long, default_value = "e2e")]
        output: Utf8PathBuf,
    },

    /// List all fixtures found in the fixtures directory.
    List {
        /// Directory containing fixture JSON files.
        #[arg(short, long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,
    },
}

/// Supported target languages for test generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Language {
    /// Generate tests for all supported languages.
    All,
    Rust,
    Python,
    Typescript,
    Go,
    Java,
    Csharp,
    Php,
    Ruby,
    Elixir,
    R,
    Wasm,
    C,
}

impl Language {
    /// Returns all concrete (non-All) languages.
    fn all_concrete() -> &'static [Language] {
        &[
            Language::Rust,
            Language::Python,
            Language::Typescript,
            Language::Go,
            Language::Java,
            Language::Csharp,
            Language::Php,
            Language::Ruby,
            Language::Elixir,
            Language::R,
            Language::Wasm,
            Language::C,
        ]
    }

    /// Returns the canonical lowercase name used in skip directives.
    const fn as_str(self) -> &'static str {
        match self {
            Language::All => "all",
            Language::Rust => "rust",
            Language::Python => "python",
            Language::Typescript => "typescript",
            Language::Go => "go",
            Language::Java => "java",
            Language::Csharp => "csharp",
            Language::Php => "php",
            Language::Ruby => "ruby",
            Language::Elixir => "elixir",
            Language::R => "r",
            Language::Wasm => "wasm",
            Language::C => "c",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { fixtures: fixtures_dir } => {
            let fixtures =
                load_fixtures(&fixtures_dir).with_context(|| format!("Failed to load fixtures from {fixtures_dir}"))?;

            println!("Found {} fixture(s):", fixtures.len());
            for fixture in &fixtures {
                println!(
                    "  [{category}] {id} — {description}",
                    category = fixture.resolved_category(),
                    id = fixture.id,
                    description = fixture.description,
                );
            }
            Ok(())
        }

        Commands::Generate {
            lang,
            fixtures: fixtures_dir,
            output,
        } => {
            let fixtures =
                load_fixtures(&fixtures_dir).with_context(|| format!("Failed to load fixtures from {fixtures_dir}"))?;

            println!("Loaded {} fixture(s) from {fixtures_dir}", fixtures.len());

            let langs_to_generate: Vec<Language> = if lang == Language::All {
                Language::all_concrete().to_vec()
            } else {
                vec![lang]
            };

            let mut total_generated = 0;

            for target in langs_to_generate {
                let count = generate_for_language(target, &fixtures, &output)
                    .with_context(|| format!("Failed to generate {} tests", target.as_str()))?;
                if count > 0 {
                    println!("  [{}] generated {count} test(s)", target.as_str());
                } else {
                    println!(
                        "  [{}] no tests to generate (all skipped or unsupported)",
                        target.as_str()
                    );
                }
                total_generated += count;
            }

            println!("Done. Total tests generated: {total_generated}");
            Ok(())
        }
    }
}

/// Dispatch generation to the appropriate language-specific generator.
fn generate_for_language(lang: Language, fixtures: &[fixtures::Fixture], output: &Utf8PathBuf) -> Result<usize> {
    let count = match lang {
        Language::Rust => rust::generate(fixtures, output),
        Language::Python => python::generate(fixtures, output),
        Language::Typescript => typescript::generate(fixtures, output),
        Language::Go => go::generate(fixtures, output),
        Language::Java => java::generate(fixtures, output),
        Language::Csharp => csharp::generate(fixtures, output),
        Language::Php => php::generate(fixtures, output),
        Language::Ruby => ruby::generate(fixtures, output),
        Language::Elixir => elixir::generate(fixtures, output),
        Language::C => c::generate(fixtures, output),
        Language::R => r::generate(fixtures, output),
        Language::Wasm => wasm::generate(fixtures, output),
        // Languages not yet implemented produce zero output without error.
        Language::All => Ok(0),
    }?;

    if count > 0 {
        run_formatter(lang, output);
    }

    Ok(count)
}

/// Run a language-specific formatter on the generated output directory.
///
/// Failures are logged as warnings but do not abort generation.
fn run_formatter(lang: Language, output_dir: &Utf8Path) {
    let result = match lang {
        Language::Rust => {
            let manifest = format!("{}/rust/Cargo.toml", output_dir);
            run_cmd("cargo", &["fmt", "--manifest-path", &manifest])
        }
        Language::Python => {
            let py_dir = format!("{}/python", output_dir);
            let _ = run_cmd("ruff", &["check", "--fix", "--quiet", &py_dir]);
            run_cmd("ruff", &["format", "--quiet", &py_dir])
        }
        Language::Typescript | Language::Wasm => {
            let ts_dir = format!("{}/{}", output_dir, lang.as_str());
            run_cmd("biome", &["format", "--write", &ts_dir])
        }
        Language::Go => {
            let go_dir = format!("{}/go", output_dir);
            run_cmd("gofmt", &["-w", &go_dir])
        }
        Language::Java => {
            // google-java-format is optional; skip silently if not installed
            Ok(())
        }
        Language::Csharp => {
            // dotnet format requires a project file; skip here as the e2e csharp dir
            // may not have a full project yet.
            Ok(())
        }
        Language::Php => {
            let php_dir = format!("{}/php", output_dir);
            run_cmd("php-cs-fixer", &["fix", "--rules=@PSR12", "--quiet", &php_dir])
        }
        Language::Ruby => {
            let rb_dir = format!("{}/ruby", output_dir);
            run_cmd("rubocop", &["-A", "--config", "packages/ruby/.rubocop.yml", &rb_dir])
        }
        Language::Elixir => {
            let ex_dir = format!("{}/elixir", output_dir);
            let pattern = format!("{}/**/*.exs", ex_dir);
            run_cmd("mix", &["format", &pattern])
        }
        Language::R => {
            // styler requires R; skip silently if not available
            Ok(())
        }
        Language::C => {
            // clang-format on generated .c files
            let c_dir = format!("{}/c", output_dir);
            run_cmd(
                "sh",
                &[
                    "-c",
                    &format!(
                        "find {c_dir} -name '*.c' -o -name '*.h' | xargs clang-format -i --style=file 2>/dev/null || true"
                    ),
                ],
            )
        }
        Language::All => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("Warning: formatter failed for {:?}: {}", lang, e);
    }
}

/// Run a command and return `Ok(())` if it exits successfully.
fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    use std::process::Command;
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(anyhow::anyhow!("{} exited with {}", cmd, s)),
        Err(e) => Err(anyhow::anyhow!("Failed to run {}: {}", cmd, e)),
    }
}
