use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use lammps_analyser::{diagnostics::Severity, format, input_script::InputScript, lints};
use serde_json::json;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "lammps-lsp-tool")]
#[command(about = "Agent-facing Diagnostic Engine v1 CLI for LAMMPS inputs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Check a LAMMPS input script for diagnostics
    Check {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Export all lint rules as JSON
    Lint {
        #[command(subcommand)]
        sub: Option<LintSub>,
    },
    /// Explain a specific lint rule
    Explain {
        /// Rule code (e.g., "LAMMPS-E100") or rule ID (e.g., "lammps.command.unknown")
        rule: String,
    },
    /// Run static analysis and export diagnostics as JSON
    #[command(name = "test-static")]
    TestStatic {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Format a LAMMPS input script (safe, idempotent)
    Fmt {
        /// Files to format
        files: Vec<PathBuf>,
        /// Write formatted output back to files
        #[arg(short = 'w', long)]
        write: bool,
        /// Check if files are formatted (exit 1 if not)
        #[arg(long)]
        check: bool,
    },
    Context {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    Complete {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    Hover {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    Symbols {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
    Fix {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
    },
}

#[derive(Debug, Subcommand)]
enum LintSub {
    /// Export all rules as JSON
    Export,
    /// Analyze a log file for LAMMPS errors
    Log { source: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { source, .. } => check(source),
        Command::Lint { sub } => lint_cmd(sub),
        Command::Explain { rule } => explain_rule(&rule),
        Command::TestStatic { source, .. } => test_static(source),
        Command::Fmt {
            files,
            write,
            check,
        } => fmt_files(&files, write, check),
        Command::Context { source, .. } => empty_operation(source, "context"),
        Command::Complete { source, .. } => empty_operation(source, "complete"),
        Command::Hover { source, .. } => empty_operation(source, "hover"),
        Command::Symbols { source, .. } => empty_operation(source, "symbols"),
        Command::Fix { source, .. } => empty_operation(source, "fix"),
    }
}

fn check(source: PathBuf) -> Result<()> {
    let text = std::fs::read_to_string(&source).context("file must be UTF-8 encoded")?;
    let diagnostics = match InputScript::new(&text) {
        Ok(script) => {
            let base_dir = source.parent();
            let extra_lints = lints::run_all_lints_with_dir(&script.ast, &text, base_dir);
            let mut all_diags = script.diagnostics;
            all_diags.extend(extra_lints);
            all_diags
                .iter()
                .map(|diagnostic| diagnostic_to_json(diagnostic, &source))
                .collect::<Vec<_>>()
        }
        Err(error) => vec![json!({
            "diagnostic_engine": "1.0",
            "code": "LAMMPS-PARSE",
            "severity": "error",
            "category": "syntax",
            "confidence": 1.0,
            "source": "lammps-lsp",
            "range": {
                "start": {"line": 0, "character": 0},
                "end": {"line": 0, "character": 1},
            },
            "software": "lammps",
            "file_type": source.extension().and_then(|item| item.to_str()).unwrap_or("input"),
            "path": source.to_string_lossy(),
            "expected": null,
            "actual": null,
            "manual_ref": null,
            "fix_hints": [],
            "blocking": true,
            "message": error.to_string(),
        })],
    };
    let blocking = diagnostics
        .iter()
        .filter(|item| {
            item.get("blocking")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .count();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "uri": format!("file://{}", source.canonicalize().unwrap_or(source.clone()).display()),
            "operation": "check",
            "ok": blocking == 0,
            "version": "1.0",
            "software": "lammps",
            "diagnostic_engine": "1.0",
            "diagnostics": diagnostics,
            "summary": {
                "count": diagnostics.len(),
                "blocking": blocking,
                "errors": diagnostics.iter().filter(|item| item.get("severity").and_then(|value| value.as_str()) == Some("error")).count(),
                "warnings": diagnostics.iter().filter(|item| item.get("severity").and_then(|value| value.as_str()) == Some("warning")).count(),
            },
        }))?
    );
    Ok(())
}

fn lint_cmd(sub: Option<LintSub>) -> Result<()> {
    match sub {
        Some(LintSub::Export) => {
            let rules = lints::export_rules_json();
            println!("{}", serde_json::to_string_pretty(&rules)?);
            Ok(())
        }
        Some(LintSub::Log { source }) => {
            let text = std::fs::read_to_string(&source).context("file must be UTF-8 encoded")?;
            let diags = lints::run_log_lints(&text);
            let json_diags: Vec<_> = diags
                .iter()
                .map(|d| diagnostic_to_json(d, &source))
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "operation": "log-lint",
                    "ok": json_diags.is_empty(),
                    "diagnostics": json_diags,
                }))?
            );
            Ok(())
        }
        None => {
            // Default: export rules
            let rules = lints::export_rules_json();
            println!("{}", serde_json::to_string_pretty(&rules)?);
            Ok(())
        }
    }
}

fn explain_rule(rule: &str) -> Result<()> {
    let rules = lints::export_rules_json();
    let rules_arr = rules["rules"]
        .as_array()
        .context("rules should be an array")?;

    let found = rules_arr.iter().find(|r| {
        r.get("code")
            .and_then(|v| v.as_str())
            .is_some_and(|c| c == rule)
            || r.get("label")
                .and_then(|v| v.as_str())
                .is_some_and(|l| l == rule)
    });

    match found {
        Some(r) => {
            println!("{}", serde_json::to_string_pretty(r)?);
            Ok(())
        }
        None => {
            eprintln!("Rule not found: {rule}");
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "error": "rule not found",
                    "rule": rule,
                    "available_codes": rules_arr.iter().filter_map(|r| r.get("code").and_then(|c| c.as_str())).collect::<Vec<_>>(),
                }))?
            );
            std::process::exit(1);
        }
    }
}

fn test_static(source: PathBuf) -> Result<()> {
    let text = std::fs::read_to_string(&source).context("file must be UTF-8 encoded")?;

    let mut all_diags = Vec::new();

    // Parse and get standard diagnostics
    if let Ok(script) = InputScript::new(&text) {
        all_diags.extend(script.diagnostics.iter().cloned());

        // Run lint checks
        let base_dir = source.parent();
        let lint_diags = lints::run_all_lints_with_dir(&script.ast, &text, base_dir);
        all_diags.extend(lint_diags);
    }

    let json_diags: Vec<_> = all_diags
        .iter()
        .map(|d| diagnostic_to_json(d, &source))
        .collect();

    let errors = json_diags
        .iter()
        .filter(|d| d.get("severity").and_then(|v| v.as_str()) == Some("error"))
        .count();
    let warnings = json_diags
        .iter()
        .filter(|d| d.get("severity").and_then(|v| v.as_str()) == Some("warning"))
        .count();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "operation": "test-static",
            "uri": format!("file://{}", source.canonicalize().unwrap_or(source.clone()).display()),
            "ok": errors == 0,
            "version": "1.0",
            "software": "lammps",
            "diagnostic_engine": "1.0",
            "diagnostics": json_diags,
            "summary": {
                "count": json_diags.len(),
                "errors": errors,
                "warnings": warnings,
            },
        }))?
    );

    Ok(())
}

fn fmt_files(files: &[PathBuf], write: bool, check: bool) -> Result<()> {
    if files.is_empty() {
        eprintln!("No files specified. Usage: lammps-lsp-tool fmt [-w] FILE...");
        std::process::exit(1);
    }

    let mut needs_formatting = false;

    for file in files {
        let text = std::fs::read_to_string(file)
            .with_context(|| format!("failed to read {}", file.display()))?;
        let formatted = format::format_lammps(&text);

        if text != formatted {
            needs_formatting = true;
            if check {
                println!(
                    "{}",
                    json!({
                        "file": file.to_string_lossy(),
                        "formatted": false,
                        "status": "needs-formatting",
                    })
                );
            } else if write {
                std::fs::write(file, &formatted)
                    .with_context(|| format!("failed to write {}", file.display()))?;
                println!(
                    "{}",
                    json!({
                        "file": file.to_string_lossy(),
                        "formatted": true,
                        "status": "formatted",
                    })
                );
            } else {
                print!("{formatted}");
            }
        } else {
            if check {
                println!(
                    "{}",
                    json!({
                        "file": file.to_string_lossy(),
                        "formatted": true,
                        "status": "already-formatted",
                    })
                );
            }
        }
    }

    if check && needs_formatting {
        std::process::exit(1);
    }

    Ok(())
}

fn empty_operation(source: PathBuf, operation: &str) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "uri": format!("file://{}", source.canonicalize().unwrap_or(source.clone()).display()),
            "operation": operation,
            "ok": true,
            "version": "1.0",
            "software": "lammps",
            "diagnostic_engine": "1.0",
            "diagnostics": [],
            "summary": {
                "count": 0,
                "blocking": 0,
                "errors": 0,
                "warnings": 0,
                "note": format!("{operation} is reserved by the Diagnostic Engine v1 CLI contract"),
            },
        }))?
    );
    Ok(())
}

fn diagnostic_to_json(
    diagnostic: &lammps_analyser::diagnostics::Diagnostic,
    source: &std::path::Path,
) -> serde_json::Value {
    let severity = severity_label(diagnostic.severity);
    json!({
        "diagnostic_engine": "1.0",
        "code": diagnostic.code.as_deref().unwrap_or(diagnostic.name),
        "severity": severity,
        "category": infer_category(diagnostic.name, &diagnostic.message),
        "confidence": 1.0,
        "source": "lammps-lsp",
        "range": {
            "start": {
                "line": diagnostic.span.start.row,
                "character": diagnostic.span.start.column,
            },
            "end": {
                "line": diagnostic.span.end.row,
                "character": diagnostic.span.end.column,
            },
        },
        "software": "lammps",
        "file_type": source.extension().and_then(|item| item.to_str()).unwrap_or("input"),
        "path": source.to_string_lossy(),
        "expected": null,
        "actual": null,
        "manual_ref": null,
        "fix_hints": [],
        "blocking": severity == "error",
        "message": diagnostic.message,
    })
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "information",
        Severity::Hint => "hint",
    }
}

fn infer_category(code: &str, message: &str) -> &'static str {
    let text = format!("{} {}", code, message).to_lowercase();
    if text.contains("syntax") || text.contains("parse") {
        "syntax"
    } else if text.contains("style") || text.contains("deprecated") {
        "style/deprecation"
    } else if text.contains("reference")
        || text.contains("variable")
        || text.contains("fix")
        || text.contains("compute")
    {
        "cross-file reference"
    } else if text.contains("argument") || text.contains("value") || text.contains("type") {
        "type/value"
    } else if text.contains("command") || text.contains("style") {
        "schema"
    } else if text.contains("missing") || text.contains("include") || text.contains("data file") {
        "file-check"
    } else if text.contains("units") || text.contains("pair_style") {
        "units/consistency"
    } else if text.contains("error") || text.contains("log") {
        "log-parser"
    } else {
        "semantic consistency"
    }
}
