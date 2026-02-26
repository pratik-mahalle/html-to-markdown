---
title: Contributing
description: How to contribute to html-to-markdown
---

# Contributing

Thank you for your interest in contributing to html-to-markdown. This guide covers development setup, testing, code quality standards, and the pull request process.

---

## Prerequisites

### Core Development

- **Rust** 1.80+ (stable) -- [install](https://rustup.rs/)
- **Python** 3.10+ -- for Python bindings and scripts
- **uv** -- Python package manager ([install](https://docs.astral.sh/uv/))
- **Task** -- Task runner ([install](https://taskfile.dev/))
- **prek** -- Pre-commit hooks (`uv tool install prek`)

### Optional (Language-Specific)

- **Node.js** 18+ and **pnpm** 10+ -- for TypeScript/WASM bindings
- **Ruby** 3.2+ with **bundler** -- for Ruby gem
- **PHP** 8.4+ with **Composer** -- for PHP extension
- **wasm-pack** -- for WASM builds (`cargo install wasm-pack`)

---

## Development Setup

Clone the repository and run the setup task:

```bash
git clone https://github.com/kreuzberg-dev/html-to-markdown.git
cd html-to-markdown
task setup
```

This will:

1. Install Python dependencies with `uv sync`
2. Build the Rust extension with maturin
3. Install prek hooks for commit linting and code quality

Install pre-commit hooks:

```bash
prek install
prek install --hook-type commit-msg
```

---

## Running Tests

### All Tests

```bash
task test
```

This runs both Rust and Python test suites.

### Rust Tests

```bash
task test:rust
```

Or directly:

```bash
cargo test --workspace
```

### Python Tests

```bash
task test:python
```

### TypeScript / WASM Tests

```bash
pnpm install
pnpm test
```

For specific packages:

```bash
pnpm run test:node      # NAPI-RS bindings
pnpm run test:wasm      # WebAssembly bindings
pnpm run test:ts        # TypeScript package
```

### Ruby Tests

```bash
cd packages/ruby
bundle exec rake compile
bundle exec rake spec
```

### Coverage

```bash
task cov:all
```

This generates Rust and Python coverage reports in lcov format.

**Coverage thresholds:**

- Rust core: 95% minimum
- Python bindings: 80% minimum
- TypeScript bindings: 80% minimum
- Ruby bindings: 80% minimum

---

## Code Quality

### Formatting and Linting

```bash
# Format all code (Rust + Python)
task format

# Run all linters
task lint
```

### Rust

- **Formatting:** `cargo fmt --all`
- **Linting:** `cargo clippy --workspace -- -D warnings` (zero warnings enforced)

### Python

- **Formatting:** ruff format (120 character line length)
- **Linting:** ruff check with strict rule set
- **Type checking:** mypy in strict mode

### TypeScript

- **Linting/Formatting:** Biome
- **Type checking:** TypeScript 5.x in strict mode

### Pre-commit Hooks

All Rust and Python checks run automatically on commit via prek. To run all hooks manually:

```bash
prek run --all-files
```

!!! warning "Use prek, not pre-commit"
    This project uses **prek** for pre-commit hooks, not the `pre-commit` tool. They are different tools -- make sure you have prek installed.

---

## Benchmarking

```bash
# Quick benchmarks
task bench

# Full benchmark suite
task bench:all
```

Performance regressions greater than 5% will fail CI. Always run benchmarks before submitting performance-related changes.

---

## Making Changes

### Rust Core Changes

1. Edit code in `crates/html-to-markdown/src/`
2. Run Rust tests: `task test:rust`
3. Rebuild language bindings as needed:
    - Python: `task build`
    - Node.js: `cd crates/html-to-markdown-node && pnpm run build`
    - WASM: `cd crates/html-to-markdown-wasm && pnpm run build:all`
4. Run integration tests across affected bindings

### Python API Changes

1. Edit code in `packages/python/html_to_markdown/`
2. Update type stubs in `_rust.pyi` if changing the API surface
3. Run tests: `task test:python`

### TypeScript / Node.js Changes

1. Edit Rust code in `crates/html-to-markdown-node/src/lib.rs`
2. Rebuild: `pnpm run build`
3. Test: `pnpm test`

### Adding Tests

- **Rust tests:** `crates/*/src/lib.rs` or `crates/*/tests/`
- **Python tests:** `packages/python/tests/` (pytest patterns)
- **TypeScript tests:** `packages/typescript/tests/` (vitest)
- **Ruby specs:** `packages/ruby/spec/`

---

## Commit Guidelines

Commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat: add support for definition lists
fix: handle nested blockquotes correctly
docs: update visitor pattern guide
refactor: simplify table cell parsing
test: add edge case tests for hOCR conversion
```

The prek commitlint hook enforces this format automatically.

---

## Pull Request Process

1. **Fork** the repository
2. **Create a feature branch:** `git checkout -b feat/amazing-feature`
3. **Make your changes** following the guidelines above
4. **Run tests and linting:**

    ```bash
    task test
    task lint
    ```

5. **Commit** with conventional commit format
6. **Push** and create a pull request

### PR Checklist

- [ ] Tests pass (`task test`)
- [ ] Linting passes (`task lint`)
- [ ] New public APIs have documentation and examples
- [ ] Breaking changes include migration notes
- [ ] Coverage thresholds are maintained

### CI Workflows

Pull requests trigger path-filtered CI workflows:

- `ci-rust` -- Rust core tests and linting
- `ci-python` -- Python binding tests
- `ci-node` -- Node.js binding tests
- `ci-wasm` -- WebAssembly binding tests
- `ci-ruby` -- Ruby binding tests
- `ci-php` -- PHP binding tests
- `ci-go` -- Go binding tests
- `ci-java` -- Java binding tests
- `ci-elixir` -- Elixir binding tests
- `ci-validate` -- Cross-cutting validation checks

All relevant workflows must pass before merging.

---

## Project Structure

```text
html-to-markdown/
├── crates/                     # Rust crates
│   ├── html-to-markdown/       # Core conversion library
│   ├── html-to-markdown-cli/   # CLI binary
│   ├── html-to-markdown-ffi/   # C FFI shared library
│   ├── html-to-markdown-node/  # NAPI-RS bindings (Node.js)
│   ├── html-to-markdown-wasm/  # wasm-bindgen (browsers)
│   ├── html-to-markdown-py/    # PyO3 bindings (Python)
│   └── html-to-markdown-php/   # ext-php-rs (PHP)
├── packages/                   # Language-specific packages
│   ├── python/                 # PyPI package
│   ├── typescript/             # npm TypeScript package
│   ├── ruby/                   # RubyGems gem
│   ├── php/                    # Composer package
│   ├── go/                     # Go module
│   ├── java/                   # Maven package
│   ├── csharp/                 # NuGet package
│   ├── elixir/                 # Hex package
│   └── r/                      # R package
├── docs/                       # Documentation (MkDocs)
├── examples/                   # Runnable examples
├── scripts/                    # Build and utility scripts
└── tools/                      # Development tools (benchmark harness)
```

---

## Getting Help

- **Issues:** [GitHub Issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Discussions:** [GitHub Discussions](https://github.com/kreuzberg-dev/html-to-markdown/discussions)
- **Discord:** [Kreuzberg Community](https://discord.gg/pXxagNK2zN)

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE).
