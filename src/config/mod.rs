//! Configuration management for Woof.
//!
//! This module handles loading and parsing configuration from TOML files.
//!
//! # Configuration File Format
//!
//! Configuration is loaded from `woof.toml` or `.woof.toml` in the current directory:
//!
//! ```toml
//! [global]
//! target_go_version = "1.21"
//! respect_gitignore = true
//! exclude = ["vendor/", "*.gen.go"]
//!
//! [linter]
//! select = ["E", "W", "D"]
//! ignore = ["E101"]
//!
//! [linter.rules.line-too-long]
//! enabled = true
//! severity = "error"
//! options = { max_length = 100 }
//!
//! [formatter]
//! use_tabs = true
//! tab_width = 4
//! line_length = 120
//! ```
//!
//! # Example
//!
//! ```
//! use woofmt::config::Config;
//! use std::path::Path;
//!
//! // Load from specific file
//! let config = Config::load(Some(Path::new("woof.toml"))).unwrap();
//!
//! // Or load from default locations
//! let config = Config::load(None).unwrap();
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Parallelism configuration for high-performance linting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelismConfig {
    /// Number of worker threads (0 = use all CPUs)
    #[serde(default = "default_num_threads")]
    pub num_threads: usize,

    /// Batch size for file processing
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Enable rule-level parallelism (in addition to file-level)
    #[serde(default = "default_true")]
    pub rule_parallelism: bool,

    /// Minimum file size (bytes) to trigger parallel rule execution
    #[serde(default = "default_parallel_threshold")]
    pub parallel_threshold: usize,

    /// CPU affinity (pin threads to cores)
    #[serde(default)]
    pub cpu_affinity: bool,

    /// Work stealing strategy ("auto", "static", "dynamic")
    #[serde(default = "default_work_steal")]
    pub work_steal: String,
}

impl Default for ParallelismConfig {
    fn default() -> Self {
        Self {
            num_threads: default_num_threads(),
            batch_size: default_batch_size(),
            rule_parallelism: true,
            parallel_threshold: default_parallel_threshold(),
            cpu_affinity: false,
            work_steal: default_work_steal(),
        }
    }
}

fn default_num_threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

fn default_batch_size() -> usize {
    50
}

fn default_parallel_threshold() -> usize {
    // 1KB - files smaller than this use sequential rule execution
    1024
}

fn default_work_steal() -> String {
    "auto".to_string()
}

/// Configuration for woof
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct Config {
    /// Global settings
    #[serde(default)]
    pub global: GlobalConfig,

    /// Linter configuration
    #[serde(default)]
    pub linter: LinterConfig,

    /// Formatter configuration
    #[serde(default)]
    pub formatter: FormatterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Target Go version
    #[serde(default = "default_go_version")]
    pub target_go_version: String,

    /// Files to exclude
    #[serde(default)]
    pub exclude: Vec<String>,

    /// Respect .gitignore files
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,

    /// Parallelism configuration
    #[serde(default)]
    pub parallelism: ParallelismConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct LinterConfig {
    /// Enable all rules by default
    #[serde(default)]
    pub select: Vec<String>,

    /// Rules to ignore
    #[serde(default)]
    pub ignore: Vec<String>,

    /// Per-rule configuration
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Whether this rule is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Rule severity (error, warning, info)
    #[serde(default)]
    pub severity: Option<String>,

    /// Rule-specific options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Use tabs for indentation
    #[serde(default = "default_true")]
    pub use_tabs: bool,

    /// Tab width (spaces per tab)
    #[serde(default = "default_tab_width")]
    pub tab_width: usize,

    /// Maximum line length
    #[serde(default = "default_line_length")]
    pub line_length: usize,

    /// Simplify code where possible
    #[serde(default)]
    pub simplify: bool,
}

fn default_go_version() -> String {
    "1.21".to_string()
}

fn default_true() -> bool {
    true
}

fn default_tab_width() -> usize {
    4
}

fn default_line_length() -> usize {
    120
}


impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            target_go_version: default_go_version(),
            exclude: vec![
                "vendor/".to_string(),
                "*.gen.go".to_string(),
                "*_test.go".to_string(),
            ],
            respect_gitignore: true,
            parallelism: ParallelismConfig::default(),
        }
    }
}


impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            use_tabs: true,
            tab_width: default_tab_width(),
            line_length: default_line_length(),
            simplify: false,
        }
    }
}

impl Config {
    /// Load configuration from file or use defaults
    pub fn load(path: Option<&Path>) -> Result<Self> {
        if let Some(config_path) = path {
            let content = std::fs::read_to_string(config_path).with_context(|| {
                format!("Failed to read config file: {}", config_path.display())
            })?;
            let config: Config = toml::from_str(&content).with_context(|| {
                format!("Failed to parse config file: {}", config_path.display())
            })?;
            return Ok(config);
        }

        // Try to find config file in current directory
        for file_name in ["woof.toml", ".woof.toml", "woof.json"] {
            if let Ok(content) = std::fs::read_to_string(file_name) {
                let config: Config = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {}", file_name))?;
                return Ok(config);
            }
        }

        Ok(Config::default())
    }

    /// Generate default configuration file content
    pub fn generate_default(strict: bool) -> String {
        if strict {
            r#"# Woof configuration (strict mode)
[global]
target_go_version = "1.21"
respect_gitignore = true

[linter]
select = ["E", "F", "W", "I"]
ignore = []

[linter.rules]
[linter.rules.line-too-long]
enabled = true
severity = "error"
options = { max_length = 100 }

[formatter]
use_tabs = true
tab_width = 4
line_length = 100
simplify = true
"#
            .to_string()
        } else {
            r#"# Woof configuration
[global]
target_go_version = "1.21"
respect_gitignore = true
exclude = ["vendor/", "*.gen.go"]

[linter]
# select = []  # Enable specific rules, empty means use defaults
# ignore = []  # Disable specific rules

[formatter]
use_tabs = true
tab_width = 4
line_length = 120
simplify = false
"#
            .to_string()
        }
    }

    /// Check if a rule is enabled
    pub fn is_rule_enabled(&self, rule_code: &str) -> bool {
        // If select is specified, only those rules are enabled
        if !self.linter.select.is_empty()
            && !self.linter.select.iter().any(|s| rule_code.starts_with(s)) {
                return false;
            }

        // If explicitly ignored, disable
        if self.linter.ignore.iter().any(|s| rule_code.starts_with(s)) {
            return false;
        }

        // Check per-rule config
        if let Some(rule_config) = self.linter.rules.get(rule_code) {
            return rule_config.enabled;
        }

        true
    }
}
