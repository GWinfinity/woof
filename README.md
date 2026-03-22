# Woof 🐕

Rust-powered Go linter — Lightning fast, zero dependencies.

Woof brings Pythonic code quality tools to the Go ecosystem. It combines the familiar interface of Ruff with Go-specific linting rules, enabling you to:

- ⚡ **Lint at Speed** — Parallel processing with Rayon for maximum throughput
- 🎨 **Format Consistently** — Opinionated Go code formatting
- 🔧 **Auto-Fix Issues** — Automatically fix common code problems
- 📦 **Single Binary** — Zero runtime dependencies, just one executable

Whether you're working on a small Go module or a large monorepo, Woof provides the tools you need to maintain code quality.

## Features

| Feature | Description |
|---|---|
| ⚡ Blazing Fast | Written in Rust, parallel file processing |
| 🔧 Auto-Fix | Automatically fix common issues with `--fix` |
| 🎯 Go-Native | Designed specifically for Go code patterns |
| 📊 JSON Output | CI-friendly output with `--format json` |
| 🔌 GitHub Actions | Native GitHub Actions annotation support |
| 📝 Configurable | TOML-based configuration (`woof.toml`) |
| 🧪 Tree-sitter | Precise parsing with tree-sitter |

## Requirements

- Rust 1.70+ (for building from source)
- Go 1.18+ (for linting Go projects)

## Installation

### From crates.io (coming soon)

```bash
cargo install woof
```

### From source

```bash
git clone git@github.com:GWinfinity/woof.git
cd woof
cargo install --path .
```

## Quick Start

```bash
# Check current directory
woof check .

# Check with auto-fix
woof check . --fix

# Format files in place
woof format .

# Initialize configuration
woof init
```

## Usage

### Linting

Check a file or directory for issues:

```bash
woof check .
woof check main.go
woof check ./...
```

Apply auto-fixes where possible:

```bash
woof check . --fix
```

### Formatting

Format files in place:

```bash
woof format .
woof format main.go
```

Check if files are formatted (CI mode):

```bash
woof format . --check
```

Output to stdout:

```bash
woof format main.go --stdout
```

### List all rules

```bash
woof rules
woof rules --all
```

## Lint Rules

| Code | Name | Description | Severity |
|---|---|---|---|
| E001 | unused-import | Detects unused import statements | Warning |
| E101 | line-too-long | Line exceeds maximum length | Error |
| E201 | trailing-whitespace | Line has trailing whitespace | Warning |
| E301 | empty-block | Block contains no statements | Error |
| E401 | mixed-tabs-spaces | Indentation uses both tabs and spaces | Error |
| D001 | exported-missing-doc | Exported identifier missing documentation | Warning |

## Configuration

Initialize a configuration file:

```bash
woof init           # Standard config
woof init --strict  # Strict mode
```

### Example `woof.toml`

```toml
# Woof configuration
[global]
target_go_version = "1.21"
respect_gitignore = true
exclude = ["vendor/", "*.gen.go"]

[linter]
select = ["E", "W", "D"]
ignore = ["E101"]

[linter.rules.line-too-long]
enabled = true
severity = "error"
options = { max_length = 100 }

[formatter]
use_tabs = true
tab_width = 4
line_length = 120
simplify = false
```

## Command Line Options

### Global Options

| Option | Description |
|---|---|
| `--all` | Enable all lint rules |
| `--ignore <RULES>` | Disable specific rules (comma-separated) |
| `--select <RULES>` | Select specific rules (comma-separated) |
| `--config <FILE>` | Configuration file path |
| `--threads <N>` | Number of threads to use |
| `--format <FORMAT>` | Output format: `text`, `json`, `github` |

### Check Options

| Option | Description |
|---|---|
| `--fix` | Apply auto-fixes where possible |
| `--exit-non-zero-on-fix` | Exit with error code if fixes applied |

### Format Options

| Option | Description |
|---|---|
| `--check` | Check if files are formatted without modifying |
| `--stdout` | Write formatted output to stdout |

## Project Structure

```
Woof/
├── 📁 src/
│   ├── 📁 rules/          # Lint rules implementation
│   ├── 📁 formatter/      # Code formatter
│   ├── 📁 config/         # Configuration handling
│   └── main.rs            # Entry point
├── 📁 benches/            # Performance benchmarks
├── 📁 testdata/           # Test fixtures
├── 📄 Cargo.toml          # Rust project config
├── 📄 woof.toml           # Example configuration
└── 📄 README.md           # This file
```

## Platform Support

| Platform | Status | Recommendation |
|---|---|---|
| Linux | ✅ Fully Supported | ⭐ Production |
| macOS | ✅ Fully Supported | ⭐ Production |
| Windows | ✅ Fully Supported | Development/Testing |

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Running benchmarks

```bash
cargo bench
```

### Adding a new rule

1. Create a new struct in `src/rules/builtin.rs`
2. Implement the `Rule` trait
3. Add the rule to `get_all_rules()` in `src/rules/mod.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See LICENSE for more information.

Made with ❤️ by GWinfinity

## Acknowledgments

- Inspired by [Ruff](https://github.com/astral-sh/ruff) — An extremely fast Python linter
- Uses [tree-sitter](https://tree-sitter.github.io/tree-sitter/) for parsing
