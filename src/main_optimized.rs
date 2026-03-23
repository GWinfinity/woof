//! Optimized main with faster cold start
//! 
//! Optimizations:
//! - Pre-initialize parser pool in background
//! - Lazy config loading
//! - Static rule registry

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::process;

use woofmt::cli::{Cli, Commands, OutputFormat};
use woofmt::config::Config;
use woofmt::linter::{apply_fixes, LintStats};

fn main() {
    // Spawn background thread for pre-initialization
    // This warms up the parser pool while CLI is being parsed
    let init_handle = std::thread::spawn(|| {
        // Access parser pool to trigger lazy initialization
        let _ = woofmt::parser::PARSER_POOL;
    });

    if let Err(e) = run() {
        eprintln!("{}: {}", "Error".red().bold(), e);
        process::exit(1);
    }

    // Ensure initialization completes (optional, for measurement)
    let _ = init_handle.join();
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration (lazy - only if needed)
    let config = if needs_config(&cli) {
        Config::load(cli.config.as_deref())?
    } else {
        Config::default()
    };

    // Set up thread pool
    if let Some(threads) = cli.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .ok();
    }

    match &cli.command {
        Some(Commands::Check {
            files,
            fix,
            exit_non_zero_on_fix,
        }) => {
            run_check(files, *fix, *exit_non_zero_on_fix, &config, &cli)?;
        }
        Some(Commands::Format {
            files,
            check,
            stdout,
        }) => {
            run_format(files, *check, *stdout, &config)?;
        }
        Some(Commands::Rules { all }) => {
            run_rules(*all, &config)?;
        }
        Some(Commands::Init { strict }) => {
            run_init(*strict)?;
        }
        None => {
            // Default to check mode with files from CLI
            let files = cli.get_files();
            run_check(&files, false, false, &config, &cli)?;
        }
    }

    Ok(())
}

fn needs_config(cli: &Cli) -> bool {
    // Check if the command actually needs configuration
    match &cli.command {
        Some(Commands::Rules { .. }) => false,
        Some(Commands::Init { .. }) => false,
        _ => true,
    }
}

fn run_check(
    files: &[std::path::PathBuf],
    fix: bool,
    exit_non_zero_on_fix: bool,
    config: &Config,
    cli: &Cli,
) -> Result<()> {
    let mut all_diagnostics = Vec::new();

    for file in files {
        let diagnostics = woofmt::lint_path(file, config)?;
        all_diagnostics.extend(diagnostics);
    }

    // Apply fixes if requested
    if fix && !all_diagnostics.is_empty() {
        apply_fixes_to_files(&all_diagnostics)?;
    }

    // Output diagnostics
    match cli.format {
        OutputFormat::Text => output_text(&all_diagnostics),
        OutputFormat::Json => output_json(&all_diagnostics)?,
        OutputFormat::Github => output_github(&all_diagnostics),
    }

    // Statistics
    if cli.statistics {
        let stats = LintStats::from_diagnostics(&all_diagnostics, files.len());
        output_stats(&stats);
    }

    // Exit code
    if !all_diagnostics.is_empty() {
        let has_errors = all_diagnostics.iter().any(|d| {
            matches!(d.severity, woofmt::Severity::Error)
        });

        if has_errors || (fix && exit_non_zero_on_fix) {
            process::exit(1);
        }
    }

    Ok(())
}

fn run_format(
    files: &[std::path::PathBuf],
    check: bool,
    stdout: bool,
    config: &Config,
) -> Result<()> {
    let mut all_unchanged = true;

    for file in files {
        if stdout && file.is_file() {
            // Read and format to stdout
            let source = std::fs::read_to_string(file)?;
            let formatted = woofmt::format_to_string(&source, config)?;
            print!("{}", formatted);
        } else {
            let result = woofmt::format_path(file, check, config)?;

            if check && !result.unchanged {
                if all_unchanged {
                    eprintln!("{}: The following files need formatting:", "Error".red().bold());
                }
                eprintln!("  - {}", file.display());
                all_unchanged = false;
            }
        }
    }

    if check && !all_unchanged {
        process::exit(1);
    }

    Ok(())
}

fn run_rules(all: bool, config: &Config) -> Result<()> {
    let rules = woofmt::rules::ALL_RULES;

    println!("{}", "Available Rules:".bold());
    println!();

    for &rule in rules {
        let enabled = if all {
            true
        } else {
            config.is_rule_enabled(rule.code())
        };

        let status = if enabled {
            "✓".green()
        } else {
            "✗".dimmed()
        };

        println!(
            "{} {} {} - {} {}",
            status,
            rule.code().cyan().bold(),
            rule.name().white(),
            rule.description(),
            if enabled {
                "".to_string()
            } else {
                "[disabled]".dimmed().to_string()
            }
        );
    }

    Ok(())
}

fn run_init(strict: bool) -> Result<()> {
    let config_path = "woof.toml";

    if std::path::Path::new(config_path).exists() {
        anyhow::bail!("Configuration file already exists: {}", config_path);
    }

    let content = Config::generate_default(strict);
    std::fs::write(config_path, content)?;

    println!(
        "{} Created {}",
        "✓".green().bold(),
        config_path.green()
    );

    Ok(())
}

fn output_text(diagnostics: &[woofmt::Diagnostic]) {
    for diag in diagnostics {
        let severity_color = match diag.severity {
            woofmt::Severity::Error => "error".red().bold(),
            woofmt::Severity::Warning => "warning".yellow().bold(),
            woofmt::Severity::Info => "info".blue().bold(),
        };

        println!(
            "{}:{}:{}: {} [{}] {}",
            diag.file_path,
            diag.line,
            diag.column,
            severity_color,
            diag.code.cyan(),
            diag.message
        );

        if let Some(fix) = &diag.fix {
            println!(
                "  {} {}",
                "-->".dimmed(),
                format!("Suggested fix: {}", fix.description).dimmed()
            );
        }
    }
}

fn output_json(diagnostics: &[woofmt::Diagnostic]) -> Result<()> {
    let json: Vec<_> = diagnostics
        .iter()
        .map(|d| {
            serde_json::json!({
                "file": d.file_path,
                "line": d.line,
                "column": d.column,
                "message": d.message,
                "code": d.code,
                "severity": format!("{}", d.severity),
                "fix": d.fix.as_ref().map(|f| {
                    serde_json::json!({
                        "description": f.description,
                        "replacement": f.replacement,
                    })
                }),
            })
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

fn output_github(diagnostics: &[woofmt::Diagnostic]) {
    // GitHub Actions annotation format
    for diag in diagnostics {
        let level = match diag.severity {
            woofmt::Severity::Error => "error",
            woofmt::Severity::Warning => "warning",
            woofmt::Severity::Info => "notice",
        };

        println!(
            "::{} file={},line={},col={}::[{}] {}",
            level,
            diag.file_path,
            diag.line,
            diag.column,
            diag.code,
            diag.message
        );
    }
}

fn output_stats(stats: &woofmt::linter::LintStats) {
    println!();
    println!("{}", "Statistics:".bold());
    println!("  Files checked: {}", stats.files_checked);
    println!("  Errors: {}", stats.errors.to_string().red());
    println!("  Warnings: {}", stats.warnings.to_string().yellow());
    println!("  Info: {}", stats.infos.to_string().blue());
    if stats.fixes_available > 0 {
        println!(
            "  Fixes available: {}",
            stats.fixes_available.to_string().green()
        );
    }
}

fn apply_fixes_to_files(diagnostics: &[woofmt::Diagnostic]) -> Result<()> {
    use std::collections::HashMap;

    // Group diagnostics by file
    let mut by_file: HashMap<&str, Vec<woofmt::Diagnostic>> = HashMap::new();
    for diag in diagnostics {
        by_file.entry(&diag.file_path).or_default().push(diag.clone());
    }

    // Apply fixes to each file
    for (file_path, file_diagnostics) in by_file {
        let source = std::fs::read_to_string(file_path)?;
        let fixed = apply_fixes(file_path, &file_diagnostics, &source)?;
        std::fs::write(file_path, fixed)?;
        println!("{} Fixed {}", "✓".green(), file_path);
    }

    Ok(())
}
