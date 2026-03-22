# Woof 🐕

An extremely fast Go linter and code formatter written in Rust, inspired by [Ruff](https://github.com/astral-sh/ruff).

> **Note:** Woof is currently in early development. The API and features are subject to change.

## Features

- ⚡ **Fast**: Written in Rust for maximum performance
- 🔧 **Linting**: Detect common Go code issues
- 🎨 **Formatting**: Format Go code consistently
- 🔌 **Extensible**: Easy to add new lint rules
- 📦 **Zero dependencies**: Single binary, no external dependencies

## Installation

### From source

```bash
cargo install --path .
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

Check if files are formatted (without modifying):

```bash
woof format . --check
```

Output formatted code to stdout:

```bash
woof format main.go --stdout
```

### Configuration

Initialize a configuration file:

```bash
woof init
```

Create a strict configuration:

```bash
woof init --strict
```

### List all rules

```bash
woof rules
woof rules --all
```

## Lint Rules

| Code | Name | Description |
|------|------|-------------|
| E001 | unused-import | Detects unused import statements |
| E101 | line-too-long | Line exceeds maximum length |
| E201 | trailing-whitespace | Line has trailing whitespace |
| E301 | empty-block | Block contains no statements |
| E401 | mixed-tabs-spaces | Indentation uses both tabs and spaces |
| D001 | exported-missing-doc | Exported identifier should have documentation comment |

## Configuration File

Woof looks for configuration in `woof.toml` or `.woof.toml`:

```toml
# Woof configuration
[global]
target_go_version = "1.21"
respect_gitignore = true
exclude = ["vendor/", "*.gen.go"]

[linter]
# Select specific rule categories
select = ["E", "W"]
# Ignore specific rules
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

```
--all                   Enable all lint rules
--ignore <RULES>        Disable specific rules (comma-separated)
--select <RULES>        Select specific rules (comma-separated)
--config <FILE>         Configuration file path
--threads <N>           Number of threads to use
--format <FORMAT>       Output format: text, json, github
```

### Check Options

```
--fix                   Apply auto-fixes where possible
--exit-non-zero-on-fix  Exit with error code if fixes applied
```

### Format Options

```
--check                 Check if files are formatted without modifying
--stdout                Write formatted output to stdout
```

## Performance

Woof is designed for speed:

- Parallel file processing using Rayon
- Efficient tree-sitter parsing
- Minimal allocations

Benchmarks coming soon!

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Adding a new rule

1. Create a new struct in `src/rules/builtin.rs`
2. Implement the `Rule` trait
3. Add the rule to `get_all_rules()` in `src/rules/mod.rs`

Example:

```rust
pub struct MyNewRule;

impl Rule for MyNewRule {
    fn code(&self) -> &'static str {
        "E999"
    }

    fn name(&self) -> &'static str {
        "my-new-rule"
    }

    fn description(&self) -> &'static str {
        "Description of what this rule checks"
    }

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {
        // Implementation
    }
}
```

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by [Ruff](https://github.com/astral-sh/ruff) - An extremely fast Python linter
- Uses [tree-sitter](https://tree-sitter.github.io/tree-sitter/) for parsing
