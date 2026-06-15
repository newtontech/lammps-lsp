use std::process::Command;

use serde_json::Value;

fn run_tool(args: &[&str]) -> Value {
    let bin = env!("CARGO_BIN_EXE_lammps-lsp-tool");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run lammps-lsp-tool");
    assert!(
        output.status.success(),
        "command {:?} failed: stderr={}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("stdout must be valid JSON")
}

#[test]
fn executable_is_available() {
    let bin = env!("CARGO_BIN_EXE_lammps-lsp-tool");
    let output = Command::new(bin)
        .arg("--help")
        .output()
        .expect("failed to run lammps-lsp-tool");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Agent-facing Diagnostic Engine"));
}

#[test]
fn language_id_is_declared() {
    let manifest = run_tool(&["capabilities"]);
    assert_eq!(manifest["languageId"].as_str(), Some("lammps"));
    assert_eq!(manifest["software"].as_str(), Some("lammps"));
}

#[test]
fn file_patterns_are_declared() {
    let manifest = run_tool(&["capabilities"]);
    let patterns = manifest["filePatterns"]
        .as_array()
        .expect("filePatterns array");
    assert!(!patterns.is_empty());
    for pattern in ["*.lmp", "*.lammps", "*.lmps"] {
        assert!(
            patterns.iter().any(|p| p.as_str() == Some(pattern)),
            "filePatterns should include {pattern}"
        );
    }
}

#[test]
fn required_capabilities_present() {
    let manifest = run_tool(&["capabilities"]);
    let capabilities = manifest["capabilities"]
        .as_array()
        .expect("capabilities array");
    let required = [
        "diagnostics",
        "rich-diagnostics",
        "source-provenance",
        "fix-preview",
        "output-log-diagnostics",
    ];
    for cap in required {
        assert!(
            capabilities.iter().any(|c| c.as_str() == Some(cap)),
            "capabilities should include {cap}"
        );
    }
}

#[test]
fn agent_cli_operations_declared() {
    let manifest = run_tool(&["capabilities"]);
    let agent_cli = manifest["agentCli"].as_object().expect("agentCli object");
    let operations = agent_cli["operations"]
        .as_array()
        .expect("operations array");
    for op in ["check", "context", "complete", "hover", "symbols", "fix"] {
        assert!(
            operations.iter().any(|o| o.as_str() == Some(op)),
            "agentCli.operations should include {op}"
        );
    }
}

#[test]
fn openqc_envelope_is_diagnostic_envelope_v1() {
    let manifest = run_tool(&["capabilities"]);
    let openqc = manifest["openqc"].as_object().expect("openqc object");
    assert_eq!(
        openqc["diagnosticEnvelope"].as_str(),
        Some("DiagnosticEnvelope/v1")
    );
}

#[test]
fn check_returns_diagnostic_envelope_v1() {
    let payload = run_tool(&["check", "tests/fixtures/valid/minimal.in"]);
    assert_eq!(
        payload["preflight_envelope"].as_str(),
        Some("DiagnosticEnvelope/v1")
    );
    assert_eq!(payload["software"].as_str(), Some("lammps"));
    assert_eq!(payload["version"].as_str(), Some("1.0"));
}

#[test]
fn check_valid_fixture_has_no_blocking() {
    let payload = run_tool(&["check", "tests/fixtures/valid/minimal.in"]);
    let blocking = payload["summary"]["blocking"].as_u64().unwrap_or(0);
    assert_eq!(
        blocking, 0,
        "minimal.in should have zero blocking diagnostics"
    );
}

#[test]
fn check_invalid_fixture_has_blocking() {
    let payload = run_tool(&["check", "tests/fixtures/invalid/unknown_command.in"]);
    let diagnostics = payload["diagnostics"]
        .as_array()
        .expect("diagnostics array");
    assert!(
        !diagnostics.is_empty(),
        "unknown_command.in should produce diagnostics"
    );
}

#[test]
fn fix_returns_actions_array() {
    let payload = run_tool(&["fix", "tests/fixtures/invalid/unknown_command.in"]);
    assert!(
        payload.get("actions").is_some(),
        "fix should return actions array"
    );
}

#[test]
fn lint_export_returns_rules() {
    let payload = run_tool(&["lint", "export"]);
    assert_eq!(payload["diagnostic_engine"].as_str(), Some("1.0"));
    let rules = payload["rules"].as_array().expect("rules array");
    assert!(!rules.is_empty(), "lint export should contain rules");
    for rule in rules {
        assert!(rule.get("code").is_some());
        assert!(rule.get("severity").is_some());
    }
}

#[test]
fn log_error_fixture_produces_diagnostic() {
    let payload = run_tool(&["lint", "log", "tests/fixtures/logs/error.log"]);
    let diagnostics = payload["diagnostics"].as_array().expect("diagnostics");
    assert!(
        !diagnostics.is_empty(),
        "error.log should produce diagnostics"
    );
    let has_e900 = diagnostics
        .iter()
        .any(|d| d["code"].as_str() == Some("LAMMPS-E900"));
    assert!(has_e900, "should contain LAMMPS-E900");
}

#[test]
fn clean_log_fixture_produces_no_diagnostics() {
    let payload = run_tool(&["lint", "log", "tests/fixtures/logs/clean.log"]);
    let diagnostics = payload["diagnostics"].as_array().expect("diagnostics");
    assert!(
        diagnostics.is_empty(),
        "clean.log should produce no diagnostics"
    );
}

#[test]
fn source_provenance_is_declared() {
    let manifest = run_tool(&["capabilities"]);
    let provenance = manifest["sourceProvenance"]
        .as_array()
        .expect("sourceProvenance array");
    assert!(!provenance.is_empty());
}

#[test]
fn diagnostic_envelope_structure_valid() {
    let payload = run_tool(&["check", "tests/fixtures/valid/minimal.in"]);

    for key in [
        "uri",
        "operation",
        "ok",
        "version",
        "software",
        "diagnostic_engine",
        "preflight_envelope",
        "diagnostics",
        "facts",
        "summary",
    ] {
        assert!(
            payload.get(key).is_some(),
            "payload missing required field: {key}"
        );
    }

    let summary = payload["summary"].as_object().expect("summary object");
    for key in ["count", "blocking", "errors", "warnings"] {
        assert!(summary.get(key).is_some(), "summary missing field: {key}");
    }
}
