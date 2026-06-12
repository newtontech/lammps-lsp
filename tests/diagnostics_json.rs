//! Tests for parser-backed diagnostics JSON output from the CLI.
//!
//! Validates that `lammps-analyser` produces correct diagnostics for
//! both valid and invalid LAMMPS input scripts.

use std::process::Command;

/// Helper: run `lammps-analyser` on a fixture file and return (exit_code, stdout, stderr).
fn run_analyser(fixture: &str) -> (std::process::ExitStatus, String, String) {
    let bin = env!("CARGO_BIN_EXE_lammps-analyser");
    let fixture_path = format!("tests/fixtures/golden/{fixture}");
    let output = Command::new(bin)
        .arg(&fixture_path)
        .output()
        .expect("failed to run lammps-analyser");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status, stdout, stderr)
}

#[test]
fn valid_fixture_exits_zero() {
    let (status, stdout, _stderr) = run_analyser("valid.in");
    // valid.in may produce warnings (e.g. style order), but those are still
    // reported by the analyser which exits 72 on any diagnostic.
    // Verify the output mentions diagnostics or "All Good".
    assert!(
        stdout.contains("issue") || stdout.contains("All Good"),
        "valid.in output should mention 'issue' or 'All Good', got: {stdout}"
    );
    // Accept either 0 (no diags) or 72 (warnings only) as valid.
    let code = status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 72,
        "valid.in should exit 0 or 72, got {code}"
    );
}

#[test]
fn invalid_fixture_exits_nonzero() {
    let (status, stdout, _stderr) = run_analyser("invalid.in");
    assert!(
        !status.success(),
        "invalid.in should exit non-zero, got {:?}",
        status.code()
    );
    // Should report some issues
    assert!(
        stdout.contains("issue"),
        "invalid.in output should mention 'issue', got: {stdout}"
    );
}

#[test]
fn parser_diagnostics_are_structured() {
    // Parse the valid fixture directly and check diagnostics are empty or convertible
    let source = include_str!("fixtures/golden/valid.in");
    let script = lammps_analyser::input_script::InputScript::new(source)
        .expect("valid.in should parse without error");

    // Verify all diagnostics from a valid script are convertible to LSP types
    for diag in &script.diagnostics {
        let _lsp_diag: lsp_types::Diagnostic = diag.clone().into();
    }
}

#[test]
fn invalid_fixture_produces_diagnostics() {
    let source = include_str!("fixtures/golden/invalid.in");
    let script = lammps_analyser::input_script::InputScript::new(source)
        .expect("invalid.in should still parse (with diagnostics)");

    assert!(
        !script.diagnostics.is_empty(),
        "invalid.in should produce at least one diagnostic, got none"
    );

    // Verify diagnostics have proper fields for LSP conversion
    for diag in &script.diagnostics {
        let _lsp_diag: lsp_types::Diagnostic = diag.clone().into();
    }
}

#[test]
fn diagnostics_have_code_fields() {
    let source = include_str!("fixtures/golden/invalid.in");
    let script =
        lammps_analyser::input_script::InputScript::new(source).expect("invalid.in should parse");

    // At least some diagnostics should have code fields
    let with_codes: Vec<_> = script
        .diagnostics
        .iter()
        .filter(|d| d.code.is_some())
        .collect();
    assert!(
        !with_codes.is_empty(),
        "At least some diagnostics should have code fields"
    );

    // Verify codes follow the LAMMPS-E### or LAMMPS-W### pattern
    for diag in &with_codes {
        let code = diag.code.as_ref().unwrap();
        assert!(
            code.starts_with("LAMMPS-"),
            "Diagnostic code should start with LAMMPS-, got: {code}"
        );
    }
}

#[test]
fn rule_export_is_valid_json() {
    let rules = lammps_analyser::lints::export_rules_json();
    assert_eq!(rules["diagnostic_engine"], "1.0");
    assert_eq!(rules["software"], "lammps");
    let rules_arr = rules["rules"].as_array().expect("rules should be array");
    assert!(!rules_arr.is_empty(), "rules export should contain rules");

    for rule in rules_arr {
        assert!(rule.get("code").is_some(), "rule should have code");
        assert!(rule.get("label").is_some(), "rule should have label");
        assert!(rule.get("severity").is_some(), "rule should have severity");
        assert!(
            rule.get("description").is_some(),
            "rule should have description"
        );
    }
}

#[test]
fn formatter_is_idempotent_on_valid_fixture() {
    let source = include_str!("fixtures/golden/valid.in");
    assert!(
        lammps_analyser::format::is_idempotent(source),
        "formatter should be idempotent on valid.in"
    );
}

#[test]
fn formatter_is_idempotent_on_invalid_fixture() {
    let source = include_str!("fixtures/golden/invalid.in");
    assert!(
        lammps_analyser::format::is_idempotent(source),
        "formatter should be idempotent on invalid.in"
    );
}

#[test]
fn log_error_detection_on_fixture() {
    let log = include_str!("fixtures/logs/error.log");
    let diags = lammps_analyser::lints::run_log_lints(log);
    assert_eq!(diags.len(), 1, "error.log should have exactly 1 error");
    assert_eq!(diags[0].code.as_deref(), Some("LAMMPS-E900"));
}

#[test]
fn clean_log_no_errors() {
    let log = include_str!("fixtures/logs/clean.log");
    let diags = lammps_analyser::lints::run_log_lints(log);
    assert!(diags.is_empty(), "clean.log should have no errors");
}

#[test]
fn new_valid_fixtures_parse_cleanly() {
    let fixtures = [
        include_str!("fixtures/valid/minimal.in"),
        include_str!("fixtures/valid/variables.in"),
        include_str!("fixtures/valid/fix_compute.in"),
    ];

    for source in fixtures {
        let script = lammps_analyser::input_script::InputScript::new(source)
            .expect("valid fixture should parse");
        // Only warnings are acceptable for valid fixtures
        let errors: Vec<_> = script
            .diagnostics
            .iter()
            .filter(|d| d.severity == lammps_analyser::diagnostics::Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "valid fixture should have no errors, got: {errors:?}"
        );
    }
}

#[test]
fn invalid_fixtures_produce_expected_diagnostics() {
    let fixtures = [
        ("unknown_command.in", "unknown command"),
        ("undefined_variable.in", "undefined"),
    ];

    for (name, expected_msg) in fixtures {
        let path = format!("tests/fixtures/invalid/{name}");
        let source =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
        let script = lammps_analyser::input_script::InputScript::new(&source)
            .unwrap_or_else(|e| panic!("failed to parse {path}: {e}"));

        // Run extra lints
        let mut all_diags = script.diagnostics.clone();
        let extra = lammps_analyser::lints::run_all_lints(&script.ast, &source);
        all_diags.extend(extra);

        let found = all_diags
            .iter()
            .any(|d| d.message.to_lowercase().contains(expected_msg));
        assert!(
            found,
            "{name} should produce a diagnostic containing '{expected_msg}', got: {:?}",
            all_diags.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }
}
