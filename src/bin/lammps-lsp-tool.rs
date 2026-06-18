//! lammps-lsp-tool: Agent-facing Diagnostic Engine v1 CLI.
//! See also: wiki/synthesis/openqc-agent-context.md
//! See also: wiki/entities/Run_Command.md
use std::collections::HashSet;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use lammps_analyser::{diagnostics::Severity, format, input_script::InputScript, lints};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const DOC_INDEX: &str = include_str!("../../docs_extract/index_map.txt");
const SKILL_YAML: &str = include_str!("../../skill/skill.yaml");
const SKILL_MD: &str = include_str!("../../skill/SKILL.md");
const SKILL_REFERENCES_README: &str = include_str!("../../skill/references/README.md");
const OPERATIONS: &[&str] = &[
    "capabilities",
    "skill-spec",
    "skill-export",
    "check",
    "context",
    "complete",
    "hover",
    "symbols",
    "fix",
    "preflight",
    "manifest",
];

#[derive(Debug, Parser)]
#[command(name = "lammps-lsp-tool")]
#[command(about = "Agent-facing Diagnostic Engine v1 CLI for LAMMPS inputs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the OpenQC LSP capability manifest as JSON
    Capabilities {
        #[arg(long, default_value = "json")]
        format: String,
    },
    /// Print the packaged pluggable skill manifest
    #[command(name = "skill-spec")]
    SkillSpec {
        #[arg(long, default_value = "yaml")]
        format: String,
    },
    /// Export the packaged pluggable skill directory
    #[command(name = "skill-export")]
    SkillExport {
        #[arg(long)]
        output: PathBuf,
    },
    /// Check a LAMMPS input script for diagnostics
    Check {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        fail_on_blocking: bool,
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
        #[arg(long, default_value_t = 0)]
        line: usize,
        #[arg(long, default_value_t = 0)]
        character: usize,
    },
    Complete {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value_t = 0)]
        line: usize,
        #[arg(long, default_value_t = 0)]
        character: usize,
    },
    Hover {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value_t = 0)]
        line: usize,
        #[arg(long, default_value_t = 0)]
        character: usize,
    },
    Symbols {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value_t = 0)]
        line: usize,
        #[arg(long, default_value_t = 0)]
        character: usize,
    },
    Fix {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long, default_value_t = 0)]
        line: usize,
        #[arg(long, default_value_t = 0)]
        character: usize,
    },
    /// Run universal generated-input preflight checks
    Preflight {
        source: PathBuf,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        fail_on_blocking: bool,
    },
    /// Export fleet preflight manifest JSON
    Manifest {
        #[arg(long)]
        source: Option<PathBuf>,
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
    let exit_code = match cli.command {
        Command::Capabilities { .. } => {
            capabilities()?;
            0
        }
        Command::SkillSpec { format } => {
            skill_spec(&format)?;
            0
        }
        Command::SkillExport { output } => {
            skill_export(&output)?;
            0
        }
        Command::Check {
            source,
            fail_on_blocking,
            ..
        } => check(source, fail_on_blocking)?,
        Command::Lint { sub } => {
            lint_cmd(sub)?;
            0
        }
        Command::Explain { rule } => {
            explain_rule(&rule)?;
            0
        }
        Command::TestStatic { source, .. } => {
            test_static(source)?;
            0
        }
        Command::Fmt {
            files,
            write,
            check,
        } => {
            fmt_files(&files, write, check)?;
            0
        }
        Command::Context {
            source,
            line,
            character,
            ..
        } => {
            agent_operation(source, "context", line, character)?;
            0
        }
        Command::Complete {
            source,
            line,
            character,
            ..
        } => {
            agent_operation(source, "complete", line, character)?;
            0
        }
        Command::Hover {
            source,
            line,
            character,
            ..
        } => {
            agent_operation(source, "hover", line, character)?;
            0
        }
        Command::Symbols {
            source,
            line,
            character,
            ..
        } => {
            agent_operation(source, "symbols", line, character)?;
            0
        }
        Command::Fix {
            source,
            line,
            character,
            ..
        } => {
            agent_operation(source, "fix", line, character)?;
            0
        }
        Command::Preflight {
            source,
            fail_on_blocking,
            ..
        } => preflight(source, fail_on_blocking)?,
        Command::Manifest { source, .. } => {
            manifest(source)?;
            0
        }
    };
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}

fn capabilities() -> Result<()> {
    if let Some(manifest) = find_manifest()? {
        println!("{}", serde_json::to_string_pretty(&manifest)?);
        return Ok(());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "OpenQCLspCapabilities",
            "version": 1,
            "software": "lammps",
            "capabilities": [
                "diagnostics",
                "rich-diagnostics",
                "completion",
                "hover",
                "symbols",
                "fix-preview",
                "llm-wiki",
                "openqc-context",
            ],
            "agentCli": {
                "operations": OPERATIONS,
                "jsonFormat": true,
                "failOnBlocking": true,
            },
        }))?
    );
    Ok(())
}

fn skill_spec(format: &str) -> Result<()> {
    match format {
        "yaml" | "yml" => {
            print!("{SKILL_YAML}");
        }
        "json" => {
            let manifest: serde_yaml::Value = serde_yaml::from_str(SKILL_YAML)?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
        other => {
            anyhow::bail!("unsupported format: {other}");
        }
    }
    Ok(())
}

fn skill_export(output: &Path) -> Result<()> {
    std::fs::create_dir_all(output.join("references"))
        .with_context(|| format!("failed to create {}", output.display()))?;
    std::fs::write(output.join("skill.yaml"), SKILL_YAML)
        .with_context(|| format!("failed to write {}", output.join("skill.yaml").display()))?;
    std::fs::write(output.join("SKILL.md"), SKILL_MD)
        .with_context(|| format!("failed to write {}", output.join("SKILL.md").display()))?;
    std::fs::write(
        output.join("references").join("README.md"),
        SKILL_REFERENCES_README,
    )
    .with_context(|| {
        format!(
            "failed to write {}",
            output.join("references").join("README.md").display()
        )
    })?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "ok": true,
            "output": output,
            "files": [
                "skill.yaml",
                "SKILL.md",
                "references/README.md",
            ],
        }))?
    );
    Ok(())
}

fn find_manifest() -> Result<Option<serde_json::Value>> {
    let mut roots = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            roots.push(parent.to_path_buf());
        }
    }
    for root in roots {
        for ancestor in root.ancestors() {
            let candidate = ancestor.join("lsp-capabilities.json");
            if candidate.exists() {
                let text = std::fs::read_to_string(&candidate)
                    .with_context(|| format!("failed to read {}", candidate.display()))?;
                return Ok(Some(serde_json::from_str(&text)?));
            }
        }
    }
    Ok(None)
}

fn check(source: PathBuf, fail_on_blocking: bool) -> Result<i32> {
    let input_path = if source.is_dir() {
        resolve_preflight_input(&source)?
    } else {
        source
    };
    let payload = build_check_payload(&input_path, "check")?;
    println!("{}", serde_json::to_string_pretty(&payload)?);
    let blocking = payload
        .get("summary")
        .and_then(|summary| summary.get("blocking"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    Ok(if fail_on_blocking && blocking > 0 {
        1
    } else {
        0
    })
}

fn preflight(source: PathBuf, fail_on_blocking: bool) -> Result<i32> {
    let payload = build_preflight_payload(&source)?;
    println!("{}", serde_json::to_string_pretty(&payload)?);
    let blocking = payload
        .get("summary")
        .and_then(|summary| summary.get("blocking"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0);
    Ok(if fail_on_blocking && blocking > 0 {
        1
    } else {
        0
    })
}

fn manifest(source: Option<PathBuf>) -> Result<()> {
    let fixtures = load_fixture_manifest(source.as_deref())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&lammps_analyser::preflight::fleet_manifest(&fixtures))?
    );
    Ok(())
}

fn build_check_payload(source: &Path, operation: &str) -> Result<Value> {
    let mut diagnostics = collect_diagnostics_json(source)?;
    let (artifacts, version_assumption) = maybe_collect_preflight(source, &mut diagnostics)?;
    Ok(base_payload(
        source,
        operation,
        diagnostics,
        artifacts,
        version_assumption,
    ))
}

fn build_preflight_payload(source: &Path) -> Result<Value> {
    let input_path = resolve_preflight_input(source)?;
    let case_dir = if source.is_dir() {
        source.to_path_buf()
    } else {
        source.parent().unwrap_or(source).to_path_buf()
    };
    let intent = lammps_analyser::preflight::load_intent(&case_dir);
    let text = std::fs::read_to_string(&input_path).context("file must be UTF-8 encoded")?;
    let script = InputScript::new(&text).context("failed to parse preflight input")?;
    let (mut diagnostics, graph) = lammps_analyser::preflight::preflight_diagnostics(
        &input_path,
        &script.ast,
        intent.as_ref(),
    );
    diagnostics = dedupe_preflight_overlap(&[], diagnostics);
    let version_assumption =
        lammps_analyser::preflight::resolve_version_assumption(intent.as_ref());
    Ok(base_payload(
        &input_path,
        "preflight",
        diagnostics,
        graph.to_json(),
        Some(version_assumption),
    ))
}

fn resolve_preflight_input(source: &Path) -> Result<PathBuf> {
    if source.is_file() {
        return Ok(source.to_path_buf());
    }
    lammps_analyser::preflight::resolve_primary_input(source)
        .ok_or_else(|| anyhow::anyhow!("no primary LAMMPS input found in {}", source.display()))
}

fn maybe_collect_preflight(
    source: &Path,
    diagnostics: &mut Vec<Value>,
) -> Result<(Vec<Value>, Option<Value>)> {
    let case_dir = if source.is_dir() {
        source.to_path_buf()
    } else if lammps_analyser::preflight::looks_like_workspace(source.parent().unwrap_or(source)) {
        source.parent().unwrap_or(source).to_path_buf()
    } else {
        return Ok((Vec::new(), None));
    };
    if !lammps_analyser::preflight::looks_like_workspace(&case_dir) {
        return Ok((Vec::new(), None));
    }
    let input_path = if source.is_file() {
        source.to_path_buf()
    } else {
        resolve_preflight_input(source)?
    };
    let intent = lammps_analyser::preflight::load_intent(&case_dir);
    let text = std::fs::read_to_string(&input_path).context("file must be UTF-8 encoded")?;
    let script = InputScript::new(&text).context("failed to parse preflight input")?;
    let (preflight, graph) = lammps_analyser::preflight::preflight_diagnostics(
        &input_path,
        &script.ast,
        intent.as_ref(),
    );
    diagnostics.extend(dedupe_preflight_overlap(diagnostics.as_slice(), preflight));
    let version_assumption =
        lammps_analyser::preflight::resolve_version_assumption(intent.as_ref());
    Ok((graph.to_json(), Some(version_assumption)))
}

fn dedupe_preflight_overlap(existing: &[Value], preflight: Vec<Value>) -> Vec<Value> {
    const OVERLAP: &[(&str, &str)] = &[("LAMMPS-E700", "LAMMPS603"), ("LAMMPS-E701", "LAMMPS602")];
    let legacy_codes: HashSet<&str> = existing
        .iter()
        .filter_map(|diag| diag.get("code").and_then(|value| value.as_str()))
        .collect();
    preflight
        .into_iter()
        .filter(|diag| {
            let code = diag
                .get("code")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            !OVERLAP.iter().any(|(legacy, preflight_code)| {
                legacy_codes.contains(legacy) && preflight_code == &code
            })
        })
        .collect()
}

fn load_fixture_manifest(source: Option<&Path>) -> Result<Vec<Value>> {
    let Some(source) = source else {
        return Ok(Vec::new());
    };
    let case_dir = if source.is_dir() {
        source.to_path_buf()
    } else {
        source.parent().unwrap_or(source).to_path_buf()
    };
    let fixtures_path = case_dir.join(".lammps-lsp").join("fixtures.json");
    if !fixtures_path.exists() {
        return Ok(Vec::new());
    }
    let payload: Value = serde_json::from_str(&std::fs::read_to_string(fixtures_path)?)?;
    Ok(match payload {
        Value::Array(items) => items,
        Value::Object(map) => map
            .get("fixtures")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        _ => Vec::new(),
    })
}

fn collect_diagnostics_json(source: &Path) -> Result<Vec<Value>> {
    let text = std::fs::read_to_string(source).context("file must be UTF-8 encoded")?;
    Ok(match InputScript::new(&text) {
        Ok(script) => {
            let base_dir = source.parent();
            let extra_lints = lints::run_all_lints_with_dir(&script.ast, &text, base_dir);
            let mut all_diags = script.diagnostics;
            all_diags.extend(extra_lints);
            all_diags
                .iter()
                .map(|diagnostic| diagnostic_to_json(diagnostic, source))
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
    })
}

fn base_payload(
    source: &Path,
    operation: &str,
    diagnostics: Vec<Value>,
    artifacts: Vec<Value>,
    version_assumption: Option<Value>,
) -> Value {
    let blocking = diagnostics
        .iter()
        .filter(|item| {
            item.get("blocking")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .count();
    let mut facts = json!({
        "artifacts": artifacts,
    });
    if let Some(version_assumption) = version_assumption {
        facts["version_assumption"] = version_assumption;
    }
    json!({
        "uri": file_uri(source),
        "operation": operation,
        "ok": blocking == 0,
        "version": "1.0",
        "software": "lammps",
        "diagnostic_engine": "1.0",
        "preflight_envelope": "DiagnosticEnvelope/v1",
        "diagnostics": diagnostics,
        "facts": facts,
        "summary": {
            "count": diagnostics.len(),
            "blocking": blocking,
            "errors": diagnostics.iter().filter(|item| item.get("severity").and_then(|value| value.as_str()) == Some("error")).count(),
            "warnings": diagnostics.iter().filter(|item| item.get("severity").and_then(|value| value.as_str()) == Some("warning")).count(),
        },
        "capabilities": {
            "operations": OPERATIONS,
            "operation": operation,
            "status": "available",
            "source": "lammps-lsp-tool",
        },
    })
}

fn agent_operation(source: PathBuf, operation: &str, line: usize, character: usize) -> Result<()> {
    let text = std::fs::read_to_string(&source).context("file must be UTF-8 encoded")?;
    let mut diagnostics = collect_diagnostics_json(&source)?;
    let (artifacts, version_assumption) = maybe_collect_preflight(&source, &mut diagnostics)?;
    let mut payload = base_payload(
        &source,
        operation,
        diagnostics.clone(),
        artifacts,
        version_assumption,
    );
    payload["position"] = json!({"line": line, "character": character});

    match operation {
        "context" => {
            payload["context"] = context_for(&text, line, character);
        }
        "complete" => {
            let items = completion_items(&text);
            payload["items"] = json!(items);
            mark_availability(
                &mut payload,
                operation,
                !items.is_empty(),
                "No completion items available.",
            );
        }
        "hover" => {
            let contents = hover_contents(&text, &diagnostics, line, character);
            payload["context"] = context_for(&text, line, character);
            payload["contents"] = contents.as_ref().map_or(Value::Null, |value| json!(value));
            mark_availability(
                &mut payload,
                operation,
                contents.is_some(),
                "No hover documentation found for this position.",
            );
        }
        "symbols" => {
            let items = document_symbols(&text);
            payload["items"] = json!(items);
            mark_availability(
                &mut payload,
                operation,
                !items.is_empty(),
                "No document symbols found.",
            );
        }
        "fix" => {
            let actions = fix_actions(&diagnostics, line, character);
            payload["actions"] = json!(actions);
            mark_availability(
                &mut payload,
                operation,
                !actions.is_empty(),
                "No safe quick-fix hints are available for current diagnostics.",
            );
        }
        _ => {}
    }

    println!("{}", serde_json::to_string_pretty(&payload)?);
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

fn file_uri(source: &Path) -> String {
    format!(
        "file://{}",
        source
            .canonicalize()
            .unwrap_or_else(|_| source.to_path_buf())
            .display()
    )
}

fn context_for(text: &str, line: usize, character: usize) -> Value {
    let lines: Vec<&str> = text.lines().collect();
    let line_index = line.min(lines.len().saturating_sub(1));
    let line_text = lines.get(line_index).copied().unwrap_or("");
    let clamped_character = character.min(line_text.len());
    let (token, start, end) = word_at(line_text, clamped_character);
    let words: Vec<&str> = line_text.split_whitespace().collect();
    json!({
        "line_text": line_text,
        "token": token,
        "command": words.first().copied().unwrap_or(""),
        "style": style_token(&words),
        "word_range": {
            "start": {"line": line_index, "character": start},
            "end": {"line": line_index, "character": end},
        },
        "before": lines[line_index.saturating_sub(3)..line_index],
        "after": lines[(line_index + 1).min(lines.len())..(line_index + 4).min(lines.len())],
    })
}

fn word_at(line_text: &str, character: usize) -> (String, usize, usize) {
    let bytes = line_text.as_bytes();
    let mut start = character.min(bytes.len());
    while start > 0 && is_word_byte(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = character.min(bytes.len());
    while end < bytes.len() && is_word_byte(bytes[end]) {
        end += 1;
    }
    if start == end {
        return (String::new(), character, character);
    }
    (line_text[start..end].to_string(), start, end)
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'/' | b'-' | b'.' | b'$' | b'{')
}

fn style_token(words: &[&str]) -> Option<String> {
    match words.first().copied() {
        Some("fix") if words.len() > 3 => Some(words[3].to_string()),
        Some("compute") if words.len() > 3 => Some(words[3].to_string()),
        Some("pair_style" | "bond_style" | "angle_style" | "dihedral_style" | "improper_style")
            if words.len() > 1 =>
        {
            Some(words[1].to_string())
        }
        _ => None,
    }
}

fn completion_items(text: &str) -> Vec<Value> {
    let mut items = Vec::new();
    for (label, doc) in doc_index_entries() {
        items.push(json!({
            "label": label,
            "detail": format!("LAMMPS manual: {doc}.html"),
            "kind": completion_kind(&label),
            "manual_ref": format!("https://docs.lammps.org/{doc}.html"),
            "source": "lammps-doc-index",
        }));
    }
    for symbol in document_symbols(text) {
        if let Some(name) = symbol.get("name").and_then(|value| value.as_str()) {
            items.push(json!({
                "label": name,
                "detail": symbol.get("detail").cloned().unwrap_or_else(|| json!("Document symbol")),
                "kind": 6,
                "source": "document",
            }));
        }
    }
    dedupe_values(items, "label")
}

fn doc_index_entries() -> Vec<(String, String)> {
    DOC_INDEX
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(label, doc)| (label.trim().to_string(), doc.trim().to_string()))
        .collect()
}

fn completion_kind(label: &str) -> u8 {
    if label.starts_with("fix ") || label.starts_with("compute ") || label.contains("_style ") {
        7
    } else {
        3
    }
}

fn hover_contents(
    text: &str,
    diagnostics: &[Value],
    line: usize,
    character: usize,
) -> Option<String> {
    if let Some(diagnostic) = diagnostics_at_position(diagnostics, line, character).first() {
        let code = diagnostic
            .get("code")
            .and_then(|value| value.as_str())
            .unwrap_or("diagnostic");
        let message = diagnostic
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        return Some(format!("{code}: {message}"));
    }
    let line_text = text.lines().nth(line).unwrap_or("");
    let (token, _, _) = word_at(line_text, character);
    if token.is_empty() {
        return None;
    }
    let words: Vec<&str> = line_text.split_whitespace().collect();
    let candidates = hover_candidates(&token, &words);
    for candidate in candidates {
        if let Some((_, doc)) = doc_index_entries()
            .into_iter()
            .find(|(label, _)| label == &candidate)
        {
            return Some(format!(
                "{candidate}\nManual: https://docs.lammps.org/{doc}.html"
            ));
        }
    }
    None
}

fn hover_candidates(token: &str, words: &[&str]) -> Vec<String> {
    let mut candidates = vec![token.to_string()];
    match words.first().copied() {
        Some("fix") if words.len() > 3 => candidates.push(format!("fix {token}")),
        Some("compute") if words.len() > 3 => candidates.push(format!("compute {token}")),
        Some(command) if command.ends_with("_style") => {
            candidates.push(format!("{command} {token}"));
        }
        _ => {}
    }
    candidates
}

fn document_symbols(text: &str) -> Vec<Value> {
    text.lines()
        .enumerate()
        .filter_map(|(line, raw)| {
            let stripped = raw.split('#').next().unwrap_or("").trim();
            if stripped.is_empty() {
                return None;
            }
            let words: Vec<&str> = stripped.split_whitespace().collect();
            let name = match words.first().copied() {
                Some("fix") if words.len() > 3 => words[1],
                Some("compute") if words.len() > 3 => words[1],
                Some("variable") if words.len() > 2 => words[1],
                Some("label") if words.len() > 1 => words[1],
                Some(command) => command,
                None => return None,
            };
            let character = raw.find(name).unwrap_or(0);
            Some(json!({
                "name": name,
                "kind": symbol_kind(words.first().copied().unwrap_or("")),
                "detail": style_token(&words),
                "range": {
                    "start": {"line": line, "character": character},
                    "end": {"line": line, "character": character + name.len()},
                },
                "selectionRange": {
                    "start": {"line": line, "character": character},
                    "end": {"line": line, "character": character + name.len()},
                },
            }))
        })
        .collect()
}

fn symbol_kind(command: &str) -> &'static str {
    match command {
        "fix" => "fix",
        "compute" => "compute",
        "variable" => "variable",
        "label" => "label",
        _ => "command",
    }
}

fn fix_actions(diagnostics: &[Value], line: usize, character: usize) -> Vec<Value> {
    let selected = diagnostics_at_position(diagnostics, line, character);
    let source = if selected.is_empty() {
        diagnostics
    } else {
        &selected
    };
    source
        .iter()
        .enumerate()
        .map(|(index, diagnostic)| {
            let code = diagnostic.get("code").and_then(|value| value.as_str()).unwrap_or("diagnostic");
            let message = diagnostic
                .get("message")
                .and_then(|value| value.as_str())
                .unwrap_or("Review diagnostic");
            json!({
                "title": format!("Review {code}: {message}"),
                "kind": "quickfix",
                "diagnostic_code": code,
                "diagnostic_range": diagnostic.get("range").cloned().unwrap_or(Value::Null),
                "confidence": diagnostic.get("confidence").cloned().unwrap_or_else(|| json!(1.0)),
                "blocking": diagnostic.get("blocking").cloned().unwrap_or(Value::Bool(false)),
                "safe_to_auto_apply": false,
                "edit": null,
                "data": {"index": index, "source": diagnostic.get("source").cloned().unwrap_or(Value::Null)},
            })
        })
        .collect()
}

fn diagnostics_at_position(diagnostics: &[Value], line: usize, character: usize) -> Vec<Value> {
    diagnostics
        .iter()
        .filter(|diagnostic| {
            let Some(range) = diagnostic.get("range") else {
                return false;
            };
            let start = range.get("start").unwrap_or(&Value::Null);
            let end = range.get("end").unwrap_or(&Value::Null);
            let start_line = start
                .get("line")
                .and_then(|value| value.as_u64())
                .unwrap_or(0) as usize;
            let start_char = start
                .get("character")
                .and_then(|value| value.as_u64())
                .unwrap_or(0) as usize;
            let end_line = end
                .get("line")
                .and_then(|value| value.as_u64())
                .unwrap_or(start_line as u64) as usize;
            let end_char = end
                .get("character")
                .and_then(|value| value.as_u64())
                .unwrap_or((start_char + 1) as u64) as usize;
            start_line <= line
                && line <= end_line
                && (line != start_line || character >= start_char)
                && (line != end_line || character <= end_char)
        })
        .cloned()
        .collect()
}

fn mark_availability(payload: &mut Value, operation: &str, available: bool, reason: &str) {
    payload["capabilities"] = json!({
        "operations": OPERATIONS,
        "operation": operation,
        "status": if available { "available" } else { "unavailable" },
        "source": "lammps-lsp-tool",
        "reason": if available { Value::Null } else { json!(reason) },
    });
    if !available {
        payload["summary"]["note"] = json!(reason);
    }
}

fn dedupe_values(items: Vec<Value>, key: &str) -> Vec<Value> {
    let mut seen = std::collections::BTreeSet::new();
    let mut result = Vec::new();
    for item in items {
        let Some(label) = item.get(key).and_then(|value| value.as_str()) else {
            continue;
        };
        if seen.insert(label.to_string()) {
            result.push(item);
        }
    }
    result
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
