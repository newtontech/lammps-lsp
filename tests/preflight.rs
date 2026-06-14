//! Universal generated-input preflight tests (issue #33).

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
        "command failed: {:?}\nstderr={}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("stdout must be JSON")
}

fn codes(payload: &Value) -> Vec<String> {
    payload
        .get("diagnostics")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|diag| diag.get("code").and_then(Value::as_str).map(str::to_string))
        .collect()
}

#[test]
fn valid_preflight_fixture_is_clean() {
    let payload = run_tool(&[
        "preflight",
        "tests/fixtures/preflight/valid_minimal",
        "--format",
        "json",
    ]);
    assert_eq!(payload.get("operation").and_then(Value::as_str), Some("preflight"));
    assert_eq!(
        payload.get("preflight_envelope").and_then(Value::as_str),
        Some("DiagnosticEnvelope/v1")
    );
    assert!(payload.get("ok").and_then(Value::as_bool).unwrap_or(false));
    assert!(payload
        .get("facts")
        .and_then(|facts| facts.get("artifacts"))
        .and_then(Value::as_array)
        .is_some());
}

#[test]
fn missing_data_fixture_emits_cross_artifact_diagnostic() {
    let payload = run_tool(&[
        "preflight",
        "tests/fixtures/preflight/missing_data",
        "--format",
        "json",
    ]);
    assert!(!payload.get("ok").and_then(Value::as_bool).unwrap_or(true));
    assert!(codes(&payload).iter().any(|code| code == "LAMMPS602"));
    let diagnostic = payload
        .get("diagnostics")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .expect("expected diagnostic");
    for key in [
        "code",
        "severity",
        "path",
        "range",
        "blocking",
        "category",
        "source_provenance",
        "fix_hints",
        "actions",
    ] {
        assert!(diagnostic.get(key).is_some(), "missing {key}");
    }
}

#[test]
fn version_keyword_fixture_records_version_assumption() {
    let payload = run_tool(&[
        "preflight",
        "tests/fixtures/preflight/version_keyword",
        "--format",
        "json",
    ]);
    assert!(codes(&payload).iter().any(|code| code == "LAMMPS606"));
    assert!(payload
        .get("facts")
        .and_then(|facts| facts.get("version_assumption"))
        .and_then(|value| value.get("software_version"))
        .is_some());
}

#[test]
fn manifest_exports_fleet_regression_surface() {
    let payload = run_tool(&["manifest", "--format", "json"]);
    assert_eq!(
        payload.get("preflight_envelope").and_then(Value::as_str),
        Some("DiagnosticEnvelope/v1")
    );
    let capabilities = payload
        .get("capabilities")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    assert!(capabilities.contains(&Value::String("cross-artifact-graph".to_string())));
    assert!(capabilities.contains(&Value::String("fleet-regression-fixtures".to_string())));
}

#[test]
fn fail_on_blocking_exits_nonzero_for_missing_data() {
    let bin = env!("CARGO_BIN_EXE_lammps-lsp-tool");
    let output = Command::new(bin)
        .args([
            "preflight",
            "tests/fixtures/preflight/missing_data",
            "--format",
            "json",
            "--fail-on-blocking",
        ])
        .output()
        .expect("failed to run lammps-lsp-tool");
    assert!(!output.status.success());
}
