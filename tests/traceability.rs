//! OpenQC traceability report contract tests.

use serde_json::Value;
use std::path::Path;

const REPORT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/reports/docstring-wiki-raw-traceability.json"
);

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn load_report() -> Value {
    let text = std::fs::read_to_string(REPORT_PATH)
        .unwrap_or_else(|error| panic!("failed to read {REPORT_PATH}: {error}"));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("invalid traceability JSON: {error}"))
}

#[test]
fn report_matches_openqc_v1_contract() {
    let report = load_report();
    assert_eq!(
        report["schemaVersion"].as_str(),
        Some("openqc.lsp.traceability.v1")
    );
    assert_eq!(report["serverId"].as_str(), Some("lammps-lsp"));
    assert!(report["repository"]
        .as_str()
        .unwrap_or_default()
        .contains("lammps-lsp"));
    assert_eq!(report["languageId"].as_str(), Some("lammps"));
    assert!(report["generatedAt"].as_str().is_some());

    let summary = &report["summary"];
    let total = summary["docstringsTotal"].as_u64().unwrap_or(0);
    let linked = summary["docstringsLinked"].as_u64().unwrap_or(0);
    assert!(total > 0, "docstring traceability must be non-empty");
    assert_eq!(linked, total);
    assert_eq!(summary["brokenWikiLinks"].as_u64(), Some(0));
    assert_eq!(summary["wikiSourcesWithoutRaw"].as_u64(), Some(0));
    assert_eq!(summary["rawManifestFailures"].as_u64(), Some(0));
    assert!(!report["docstrings"].as_array().unwrap().is_empty());
    assert!(!report["wikiSources"].as_array().unwrap().is_empty());
    assert!(!report["sourceUrls"].as_array().unwrap().is_empty());
    assert_eq!(report["rawManifest"]["ok"].as_bool(), Some(true));
}

#[test]
fn report_paths_exist_and_urls_are_non_empty() {
    let report = load_report();
    for entry in report["docstrings"].as_array().unwrap() {
        assert!(root().join(entry["path"].as_str().unwrap()).is_file());
        assert!(root().join(entry["wikiPath"].as_str().unwrap()).is_file());
        assert!(!entry["symbol"].as_str().unwrap().is_empty());
    }
    for entry in report["wikiSources"].as_array().unwrap() {
        assert!(root().join(entry["wikiPath"].as_str().unwrap()).is_file());
        assert!(root().join(entry["rawPath"].as_str().unwrap()).is_file());
        assert!(!entry["sourceUrl"].as_str().unwrap().is_empty());
    }
    for entry in report["sourceUrls"].as_array().unwrap() {
        assert!(root().join(entry["rawPath"].as_str().unwrap()).is_file());
        assert!(!entry["url"].as_str().unwrap().is_empty());
    }
}
