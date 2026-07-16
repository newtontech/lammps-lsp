use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::Value;

const PACKAGE_NAME: &str = "lammps-lsp";
const VERSION: &str = "0.1.0-pre.4";
const REPOSITORY: &str = "https://github.com/newtontech/lammps-lsp";
const COMMANDS: [(&str, &str); 4] = [
    (env!("CARGO_BIN_EXE_lammps-analyser"), "lammps-analyser"),
    (env!("CARGO_BIN_EXE_lammps-lsp"), "lammps-lsp"),
    (env!("CARGO_BIN_EXE_lammps-lsp-tool"), "lammps-lsp-tool"),
    (env!("CARGO_BIN_EXE_lmp-lsp"), "lmp-lsp"),
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run_with_timeout(program: &str, args: &[&str]) -> Output {
    run_with_timeout_in(program, args, None)
}

fn run_with_timeout_in(
    program: &str,
    args: &[&str],
    current_dir: Option<&std::path::Path>,
) -> Output {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(current_dir) = current_dir {
        command.current_dir(current_dir);
    }
    let mut child = command
        .spawn()
        .unwrap_or_else(|error| panic!("failed to run {program}: {error}"));
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if child.try_wait().expect("failed to poll child").is_some() {
            return child.wait_with_output().expect("failed to collect output");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{program} {args:?} did not exit within 3 seconds");
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn run_agent(args: &[&str]) -> Output {
    run_with_timeout(env!("CARGO_BIN_EXE_lammps-lsp-tool"), args)
}

fn parse_stdout(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout was not JSON: {error}\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

#[test]
fn cargo_package_metadata_matches_the_release_identity() {
    assert_eq!(env!("CARGO_PKG_NAME"), PACKAGE_NAME);
    assert_eq!(env!("CARGO_PKG_VERSION"), VERSION);
    assert_eq!(env!("CARGO_PKG_REPOSITORY"), REPOSITORY);
}

#[test]
fn every_shipped_command_reports_the_release_version() {
    for (binary, command) in COMMANDS {
        let output = run_with_timeout(binary, &["--version"]);
        assert!(
            output.status.success(),
            "{command} --version failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            format!("{command} {VERSION}")
        );
    }
}

#[test]
fn capabilities_report_package_version_and_release_provenance() {
    let output = run_agent(&["capabilities"]);
    assert!(output.status.success());
    let manifest = parse_stdout(&output);
    assert_eq!(
        manifest["repository"].as_str(),
        Some("newtontech/lammps-lsp")
    );
    assert_eq!(manifest["packageName"].as_str(), Some(PACKAGE_NAME));
    assert_eq!(manifest["packageVersion"].as_str(), Some(VERSION));
    assert_eq!(
        manifest["releaseProvenance"]["status"].as_str(),
        Some("candidate")
    );
    assert_eq!(
        manifest["releaseProvenance"]["artifactLock"].as_str(),
        Some("release/artifacts.json")
    );

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    let isolated_dir = std::env::temp_dir().join(format!("lammps-agent-{unique}"));
    fs::create_dir(&isolated_dir).expect("failed to create isolated agent directory");
    let isolated_agent = isolated_dir.join("lammps-lsp-tool");
    fs::copy(env!("CARGO_BIN_EXE_lammps-lsp-tool"), &isolated_agent)
        .expect("failed to copy isolated agent binary");
    let isolated = run_with_timeout_in(
        isolated_agent
            .to_str()
            .expect("isolated path must be UTF-8"),
        &["capabilities"],
        Some(&isolated_dir),
    );
    fs::remove_dir_all(&isolated_dir).expect("failed to clean isolated agent directory");
    assert!(isolated.status.success());
    let embedded_manifest = parse_stdout(&isolated);
    assert_eq!(
        embedded_manifest["packageName"].as_str(),
        Some(PACKAGE_NAME)
    );
    assert_eq!(embedded_manifest["packageVersion"].as_str(), Some(VERSION));
    assert_eq!(
        embedded_manifest["repository"].as_str(),
        Some("newtontech/lammps-lsp")
    );
}

#[test]
fn release_smoke_covers_valid_invalid_and_log_fixtures() {
    let valid = run_agent(&[
        "check",
        "tests/fixtures/valid/minimal.in",
        "--fail-on-blocking",
    ]);
    assert!(valid.status.success());
    assert_eq!(
        parse_stdout(&valid)["summary"]["blocking"].as_u64(),
        Some(0)
    );

    let invalid = run_agent(&[
        "check",
        "tests/fixtures/invalid/unknown_command.in",
        "--fail-on-blocking",
    ]);
    assert!(!invalid.status.success());
    assert!(parse_stdout(&invalid)["summary"]["blocking"]
        .as_u64()
        .is_some_and(|count| count > 0));

    let log = run_agent(&["lint", "log", "tests/fixtures/logs/error.log"]);
    assert!(log.status.success());
    assert!(parse_stdout(&log)["diagnostics"]
        .as_array()
        .is_some_and(|diagnostics| diagnostics
            .iter()
            .any(|diagnostic| diagnostic["code"] == "LAMMPS-E900")));
}

#[test]
fn artifact_lock_matches_the_github_only_cargo_dist_workflow() {
    let root = repository_root();
    let artifact_lock: Value = serde_json::from_str(
        &fs::read_to_string(root.join("release/artifacts.json"))
            .expect("release artifact lock must exist"),
    )
    .expect("release artifact lock must be valid JSON");
    assert_eq!(
        artifact_lock["package"]["name"].as_str(),
        Some(PACKAGE_NAME)
    );
    assert_eq!(artifact_lock["package"]["version"].as_str(), Some(VERSION));
    assert_eq!(
        artifact_lock["cargoDist"]["version"].as_str(),
        Some("0.22.1")
    );
    assert_eq!(
        artifact_lock["cargoDist"]["publish"].as_str(),
        Some("github-release-only")
    );
    assert_eq!(artifact_lock["targets"].as_array().map(Vec::len), Some(4));

    let workflow = fs::read_to_string(root.join(".github/workflows/release.yml"))
        .expect("release workflow must exist");
    assert!(workflow.contains("v0.22.1/cargo-dist-installer.sh"));
    assert!(workflow.contains("python3 scripts/verify-release-artifacts.py artifacts"));
    assert!(!workflow.contains("cargo publish"));
}

#[test]
fn artifact_verifier_checks_contract_and_rejects_an_empty_release() {
    let root = repository_root();
    let verifier = root.join("scripts/verify-release-artifacts.py");
    let contract = Command::new("python3")
        .arg(&verifier)
        .arg("--contract-only")
        .current_dir(&root)
        .output()
        .expect("failed to run release artifact verifier");
    assert!(
        contract.status.success(),
        "contract verification failed: {}",
        String::from_utf8_lossy(&contract.stderr)
    );

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    let empty_release = std::env::temp_dir().join(format!("lammps-release-{unique}"));
    fs::create_dir(&empty_release).expect("failed to create empty release fixture");
    let missing = Command::new("python3")
        .arg(&verifier)
        .arg(&empty_release)
        .current_dir(&root)
        .output()
        .expect("failed to run release artifact verifier");
    fs::remove_dir_all(&empty_release).expect("failed to clean release fixture");
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("missing release assets"));
}
