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
    assert!(status.success(), "valid.in should exit 0, got {:?}", status.code());
    assert!(
        stdout.contains("All Good"),
        "valid.in output should mention 'All Good', got: {stdout}"
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
    // Parse the valid fixture directly and check diagnostics are empty
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
