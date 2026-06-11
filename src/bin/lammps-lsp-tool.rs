use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use lammps_analyser::{diagnostics::Severity, input_script::InputScript};
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
    Check {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { source, .. } => check(source),
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
        Ok(script) => script
            .diagnostics
            .iter()
            .map(|diagnostic| {
                let severity = severity_label(diagnostic.severity);
                json!({
                    "diagnostic_engine": "1.0",
                    "code": diagnostic.name,
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
            })
            .collect::<Vec<_>>(),
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
        .filter(|item| item.get("blocking").and_then(|value| value.as_bool()).unwrap_or(false))
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
    } else if text.contains("reference") || text.contains("variable") || text.contains("fix") || text.contains("compute") {
        "cross-file reference"
    } else if text.contains("argument") || text.contains("value") || text.contains("type") {
        "type/value"
    } else if text.contains("command") || text.contains("style") {
        "schema"
    } else {
        "semantic consistency"
    }
}
