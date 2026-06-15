//! Closed-loop fixture tests for issue #38.
//!
//! Validates that the agent CLI produces correct DiagnosticEnvelope/v1 JSON
//! for valid, invalid, and log fixtures, including no false positives on
//! valid fixtures.

use std::process::Command;

use serde_json::Value;

fn run_tool(args: &[&str]) -> (std::process::ExitStatus, Value) {
    let bin = env!("CARGO_BIN_EXE_lammps-lsp-tool");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run lammps-lsp-tool");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let json: Value = if stdout.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_str(&stdout)
            .unwrap_or_else(|e| panic!("failed to parse JSON from stdout: {e}\nstdout: {stdout}"))
    };
    (output.status, json)
}

fn diagnostics(payload: &Value) -> Vec<Value> {
    payload
        .get("diagnostics")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn codes(payload: &Value) -> Vec<String> {
    diagnostics(payload)
        .iter()
        .filter_map(|d| d.get("code").and_then(Value::as_str).map(str::to_string))
        .collect()
}

// --- Valid fixtures: must produce zero errors ---

#[test]
fn valid_minimal_fixture_is_clean() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/minimal.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should succeed on minimal.in");
    assert_eq!(
        payload.get("preflight_envelope").and_then(Value::as_str),
        Some("DiagnosticEnvelope/v1")
    );
    let errors: Vec<Value> = diagnostics(&payload)
        .into_iter()
        .filter(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .collect();
    assert!(
        errors.is_empty(),
        "minimal.in should have no errors, got: {errors:?}"
    );
}

#[test]
fn valid_variables_fixture_is_clean() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/variables.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should succeed on variables.in");
    let errors: Vec<Value> = diagnostics(&payload)
        .into_iter()
        .filter(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .collect();
    assert!(
        errors.is_empty(),
        "variables.in should have no errors, got: {errors:?}"
    );
}

#[test]
fn valid_fix_compute_fixture_is_clean() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/fix_compute.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should succeed on fix_compute.in");
    let errors: Vec<Value> = diagnostics(&payload)
        .into_iter()
        .filter(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .collect();
    assert!(
        errors.is_empty(),
        "fix_compute.in should have no errors, got: {errors:?}"
    );
}

#[test]
fn valid_lj_melt_fixture_is_clean() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/lj_melt.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should succeed on lj_melt.in");
    let errors: Vec<Value> = diagnostics(&payload)
        .into_iter()
        .filter(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .collect();
    assert!(
        errors.is_empty(),
        "lj_melt.in should have no errors, got: {errors:?}"
    );
}

#[test]
fn valid_minimize_fixture_is_clean() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/minimize.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should succeed on minimize.in");
    let errors: Vec<Value> = diagnostics(&payload)
        .into_iter()
        .filter(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .collect();
    assert!(
        errors.is_empty(),
        "minimize.in should have no errors, got: {errors:?}"
    );
}

// --- Invalid fixtures: must produce errors ---

#[test]
fn invalid_unknown_command_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/unknown_command.in",
        "--format",
        "json",
    ]);
    // Should still exit successfully (diagnostic, not crash)
    assert!(
        status.success(),
        "check should complete on unknown_command.in"
    );
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "unknown_command.in should produce diagnostics"
    );
    assert!(
        codes(&payload)
            .iter()
            .any(|c| c.contains("unknown") || c.contains("LAMMPS")),
        "should have unknown command diagnostic, got: {:?}",
        codes(&payload)
    );
}

#[test]
fn invalid_unknown_style_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/unknown_style.in",
        "--format",
        "json",
    ]);
    assert!(
        status.success(),
        "check should complete on unknown_style.in"
    );
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "unknown_style.in should produce diagnostics"
    );
}

#[test]
fn invalid_wrong_argument_count_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/wrong_argument_count.in",
        "--format",
        "json",
    ]);
    assert!(
        status.success(),
        "check should complete on wrong_argument_count.in"
    );
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "wrong_argument_count.in should produce diagnostics"
    );
}

#[test]
fn invalid_missing_data_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/missing_data.in",
        "--format",
        "json",
    ]);
    assert!(status.success(), "check should complete on missing_data.in");
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "missing_data.in should produce diagnostics"
    );
    assert!(
        codes(&payload).iter().any(|c| c.contains("LAMMPS")),
        "should have LAMMPS diagnostic, got: {:?}",
        codes(&payload)
    );
}

#[test]
fn invalid_missing_include_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/missing_include.in",
        "--format",
        "json",
    ]);
    assert!(
        status.success(),
        "check should complete on missing_include.in"
    );
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "missing_include.in should produce diagnostics"
    );
}

#[test]
fn invalid_undefined_variable_produces_diagnostic() {
    let (status, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/undefined_variable.in",
        "--format",
        "json",
    ]);
    assert!(
        status.success(),
        "check should complete on undefined_variable.in"
    );
    let diags = diagnostics(&payload);
    assert!(
        !diags.is_empty(),
        "undefined_variable.in should produce diagnostics"
    );
    assert!(
        codes(&payload)
            .iter()
            .any(|c| c.contains("undefined") || c.contains("LAMMPS")),
        "should have undefined variable diagnostic, got: {:?}",
        codes(&payload)
    );
}

// --- DiagnosticEnvelope/v1 structure validation ---

#[test]
fn diagnostic_envelope_structure_is_valid() {
    let (_, payload) = run_tool(&[
        "check",
        "tests/fixtures/valid/minimal.in",
        "--format",
        "json",
    ]);

    // Verify top-level envelope fields
    assert_eq!(payload.get("version").and_then(Value::as_str), Some("1.0"));
    assert_eq!(
        payload.get("software").and_then(Value::as_str),
        Some("lammps")
    );
    assert_eq!(
        payload.get("preflight_envelope").and_then(Value::as_str),
        Some("DiagnosticEnvelope/v1")
    );
    assert_eq!(
        payload.get("diagnostic_engine").and_then(Value::as_str),
        Some("1.0")
    );

    // Verify diagnostics array exists
    assert!(payload
        .get("diagnostics")
        .and_then(Value::as_array)
        .is_some());

    // Verify summary exists
    let summary = payload.get("summary").expect("summary should exist");
    assert!(summary.get("count").is_some());
    assert!(summary.get("blocking").is_some());
    assert!(summary.get("errors").is_some());
    assert!(summary.get("warnings").is_some());
}

#[test]
fn error_diagnostic_has_required_fields() {
    let (_, payload) = run_tool(&[
        "check",
        "tests/fixtures/invalid/unknown_command.in",
        "--format",
        "json",
    ]);

    let diags = diagnostics(&payload);
    let error_diag = diags
        .iter()
        .find(|d| d.get("severity").and_then(Value::as_str) == Some("error"))
        .expect("should have at least one error diagnostic");

    // Verify required DiagnosticEnvelope/v1 fields
    for key in [
        "code",
        "severity",
        "category",
        "confidence",
        "source",
        "range",
        "software",
        "file_type",
        "path",
        "fix_hints",
        "blocking",
        "message",
    ] {
        assert!(
            error_diag.get(key).is_some(),
            "error diagnostic missing required field: {key}"
        );
    }

    // Verify range structure
    let range = error_diag.get("range").expect("range should exist");
    assert!(range.get("start").is_some());
    assert!(range.get("end").is_some());
    let start = range.get("start").expect("start should exist");
    assert!(start.get("line").is_some());
    assert!(start.get("character").is_some());
}

// --- Log diagnostics ---

#[test]
fn log_error_fixture_produces_diagnostic() {
    let (status, payload) = run_tool(&["lint", "log", "tests/fixtures/logs/error.log"]);
    assert!(status.success(), "lint log should succeed");
    let diags = diagnostics(&payload);
    assert!(!diags.is_empty(), "error.log should produce diagnostics");
    assert!(
        codes(&payload).iter().any(|c| c == "LAMMPS-E900"),
        "should have LAMMPS-E900 code, got: {:?}",
        codes(&payload)
    );
}

#[test]
fn clean_log_fixture_produces_no_diagnostics() {
    let (status, payload) = run_tool(&["lint", "log", "tests/fixtures/logs/clean.log"]);
    assert!(status.success(), "lint log should succeed");
    let diags = diagnostics(&payload);
    assert!(
        diags.is_empty(),
        "clean.log should have no diagnostics, got: {diags:?}"
    );
}

// --- Rules export ---

#[test]
fn rules_export_has_required_fields() {
    let (status, payload) = run_tool(&["lint", "export"]);
    assert!(status.success(), "lint export should succeed");
    assert_eq!(
        payload.get("diagnostic_engine").and_then(Value::as_str),
        Some("1.0")
    );
    assert_eq!(
        payload.get("software").and_then(Value::as_str),
        Some("lammps")
    );
    let rules = payload
        .get("rules")
        .and_then(Value::as_array)
        .expect("rules should be array");
    assert!(!rules.is_empty(), "rules should not be empty");

    for rule in rules {
        assert!(rule.get("code").is_some(), "rule missing code");
        assert!(rule.get("label").is_some(), "rule missing label");
        assert!(rule.get("severity").is_some(), "rule missing severity");
        assert!(
            rule.get("description").is_some(),
            "rule missing description"
        );
    }
}

// --- Fix preview ---

#[test]
fn fix_preview_returns_actions() {
    let (_, payload) = run_tool(&[
        "fix",
        "tests/fixtures/invalid/unknown_command.in",
        "--format",
        "json",
    ]);
    // Fix should return actions array (may be empty if no fix hints)
    assert!(
        payload.get("actions").is_some(),
        "fix should return actions array"
    );
}
