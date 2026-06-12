//! MatMaster-domain execution rules for LAMMPS input scripts.
//!
//! These are newtontech-specific checks that complement the upstream
//! `lammps-analyser` diagnostics.  They are enabled by placing a
//! `.lammps-lsp/matmaster.json` file at the project root.
//!
//! # Diagnostic codes
//!
//! | Code    | Severity | Description                                   |
//! |---------|----------|-----------------------------------------------|
//! | LMP800  | error    | `pair_style` must be set before `pair_coeff`  |
//! | LMP801  | warning  | `run` timestep should be positive              |
//! | LMP802  | warning  | `include` path escapes the project root        |

use std::path::Path;

use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::spans::Span;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// MatMaster configuration loaded from `.lammps-lsp/matmaster.json`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatMasterConfig {
    /// Master switch.  When `false` no MatMaster checks run.
    pub enabled: bool,

    /// Emit LMP800 when `pair_coeff` appears before an applicable `pair_style`.
    #[serde(default = "default_true")]
    pub check_pair_coeff_order: bool,

    /// Emit LMP801 when `run` has a non-positive timestep.
    #[serde(default = "default_true")]
    pub require_positive_run: bool,

    /// Emit LMP802 when `include` paths use `..` to escape the project root.
    #[serde(default = "default_true")]
    pub forbid_parent_paths: bool,
}

const fn default_true() -> bool {
    true
}

impl MatMasterConfig {
    /// Load config from `project_root/.lammps-lsp/matmaster.json`.
    /// Returns `None` if the file does not exist or cannot be parsed.
    pub fn load(project_root: &Path) -> Option<Self> {
        let config_path = project_root.join(".lammps-lsp").join("matmaster.json");
        let content = std::fs::read_to_string(&config_path).ok()?;
        let config: Self = serde_json::from_str(&content).ok()?;
        if !config.enabled {
            return None;
        }
        Some(config)
    }
}

// ---------------------------------------------------------------------------
// Issue types
// ---------------------------------------------------------------------------

/// A MatMaster-specific issue found in a LAMMPS input script.
#[derive(Debug, Clone)]
pub struct MatMasterIssue {
    pub code: &'static str,
    pub severity: Severity,
    pub span: Span,
    pub message: String,
}

impl Issue for MatMasterIssue {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            name: self.code,
            severity: self.severity,
            span: self.span,
            message: self.message.clone(),
            code: Some(self.code.to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Checks
// ---------------------------------------------------------------------------

/// Run all enabled MatMaster checks against a LAMMPS input script.
///
/// `source` is the full text of the script.  `config` comes from
/// [`MatMasterConfig::load`].  Returns zero or more [`Diagnostic`] values.
pub fn check_source(source: &str, config: &MatMasterConfig) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();

    if config.check_pair_coeff_order {
        diagnostics.extend(check_pair_coeff_order(&lines));
    }
    if config.require_positive_run {
        diagnostics.extend(check_run_positive(&lines));
    }
    if config.forbid_parent_paths {
        diagnostics.extend(check_include_paths(&lines));
    }

    diagnostics
}

/// LMP800: `pair_style` must appear before `pair_coeff`.
fn check_pair_coeff_order(lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut saw_pair_style = false;
    // Track the line where pair_style first appears so we can return it as
    // evidence (future enhancement).  For now we simply flag pair_coeff that
    // appear before any pair_style.
    for (i, line) in lines.iter().enumerate() {
        let stripped = strip_comment(line);
        if stripped.is_empty() {
            continue;
        }
        let lower = stripped.to_lowercase();
        if lower.starts_with("pair_style") {
            saw_pair_style = true;
        } else if lower.starts_with("pair_coeff") && !saw_pair_style {
            let col = line.find("pair_coeff").unwrap_or(0);
            diagnostics.push(
                MatMasterIssue {
                    code: "LMP800",
                    severity: Severity::Error,
                    span: Span {
                        start: (i, col).into(),
                        end: (i, col + "pair_coeff".len()).into(),
                    },
                    message:
                        "pair_style must be set before pair_coeff (MatMaster execution contract)"
                            .into(),
                }
                .diagnostic(),
            );
        }
    }
    diagnostics
}

/// LMP801: `run` command should have a positive timestep.
fn check_run_positive(lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let stripped = strip_comment(line);
        if stripped.is_empty() {
            continue;
        }
        let lower = stripped.to_lowercase();
        if !lower.starts_with("run") {
            continue;
        }
        // "run" followed by a space or tab indicates the command
        let rest = if lower.len() > 3 && lower.as_bytes().get(3).copied() == Some(b' ') {
            &lower[4..]
        } else {
            continue;
        };
        // Split on whitespace, take first token
        let arg = rest.split_whitespace().next().unwrap_or("");
        // Skip pre-variable and expression forms: run ${...}, run $(...)
        if arg.starts_with("${") || arg.starts_with("$(") {
            continue;
        }
        // Try parsing as integer
        if let Ok(val) = arg.parse::<i64>() {
            if val <= 0 {
                let col = line.find("run").unwrap_or(0);
                diagnostics.push(
                    MatMasterIssue {
                        code: "LMP801",
                        severity: Severity::Warning,
                        span: Span {
                            start: (i, col).into(),
                            end: (i, col + "run".len()).into(),
                        },
                        message: format!(
                            "run timestep should be positive (MatMaster safety check), got {val}"
                        ),
                    }
                    .diagnostic(),
                );
            }
        }
    }
    diagnostics
}

/// LMP802: `include` path uses `..` to escape the project root.
fn check_include_paths(lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let stripped = strip_comment(line);
        if stripped.is_empty() {
            continue;
        }
        let lower = stripped.to_lowercase();
        if !lower.starts_with("include") {
            continue;
        }
        // Extract the path argument (first token after "include")
        let rest = if lower.len() > 7 && lower.as_bytes().get(7).copied() == Some(b' ') {
            &line[line.find("include").unwrap() + 8..] // keep original case for path
        } else {
            continue;
        };
        let path = rest.split_whitespace().next().unwrap_or("").trim();
        if path.contains("..") {
            let col = line.find("include").unwrap_or(0);
            diagnostics.push(
                MatMasterIssue {
                    code: "LMP802",
                    severity: Severity::Warning,
                    span: Span {
                        start: (i, col).into(),
                        end: (i, col + "include".len()).into(),
                    },
                    message: format!(
                        "include path '{path}' uses '..' which escapes the project root \
                     (MatMaster security check)"
                    ),
                }
                .diagnostic(),
            );
        }
    }
    diagnostics
}

/// Walk up from `file_path` looking for a `.lammps-lsp` directory.
/// Returns the ancestor directory that contains `.lammps-lsp`, or `None`.
pub fn find_project_root(file_path: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut dir: Option<&std::path::Path> = file_path.parent();
    while let Some(d) = dir {
        if d.join(".lammps-lsp").is_dir() {
            return Some(d.to_path_buf());
        }
        dir = d.parent();
    }
    None
}

/// Convert an LSP `Url` to a project root by walking up the parent tree
/// looking for a `.lammps-lsp` directory.
pub fn find_project_root_from_uri(uri: &url::Url) -> Option<std::path::PathBuf> {
    let file_path = uri.to_file_path().ok()?;
    find_project_root(&file_path)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Remove inline comment (text after `#`) and trim whitespace.
fn strip_comment(line: &str) -> &str {
    if let Some(pos) = line.find('#') {
        line[..pos].trim()
    } else {
        line.trim()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Config ----

    #[test]
    fn load_config_disabled() {
        let tmp = std::env::temp_dir();
        // No config file → no matmaster checks
        assert!(MatMasterConfig::load(&tmp).is_none());
    }

    #[test]
    fn load_config_from_valid_file() {
        let tmp = std::env::temp_dir();
        let dir = tmp.join(".lammps-lsp");
        std::fs::create_dir_all(&dir).ok();
        let cfg_path = dir.join("matmaster.json");
        std::fs::write(
            &cfg_path,
            r#"{"enabled": true, "check_pair_coeff_order": false}"#,
        )
        .unwrap();
        let cfg = MatMasterConfig::load(&tmp);
        // Clean up
        std::fs::remove_file(&cfg_path).ok();
        std::fs::remove_dir(&dir).ok();

        assert!(cfg.is_some());
        let cfg = cfg.unwrap();
        assert!(cfg.enabled);
        assert!(!cfg.check_pair_coeff_order);
        assert!(cfg.require_positive_run);
        assert!(cfg.forbid_parent_paths);
    }

    // ---- LMP800 ----

    #[test]
    fn pair_coeff_without_pair_style() {
        let source = "pair_coeff * * Cu_u3.eam\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: true,
            require_positive_run: false,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].name, "LMP800");
    }

    #[test]
    fn pair_coeff_after_pair_style_no_diag() {
        let source = "pair_style eam/alloy\npair_coeff * * Cu_u3.eam\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: true,
            require_positive_run: false,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        let lmp800: Vec<_> = diags.iter().filter(|d| d.name == "LMP800").collect();
        assert_eq!(lmp800.len(), 0);
    }

    #[test]
    fn multiple_pair_coeff_without_style() {
        let source = "pair_coeff * * Cu.eam\npair_coeff * * Al.eam\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: true,
            require_positive_run: false,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        let lmp800: Vec<_> = diags.iter().filter(|d| d.name == "LMP800").collect();
        assert_eq!(lmp800.len(), 2);
    }

    // ---- LMP801 ----

    #[test]
    fn run_negative_timestep() {
        let source = "run -10\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: false,
            require_positive_run: true,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        let lmp801: Vec<_> = diags.iter().filter(|d| d.name == "LMP801").collect();
        assert_eq!(lmp801.len(), 1);
    }

    #[test]
    fn run_positive_no_diag() {
        let source = "run 1000\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: false,
            require_positive_run: true,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        let lmp801: Vec<_> = diags.iter().filter(|d| d.name == "LMP801").collect();
        assert!(lmp801.is_empty());
    }

    #[test]
    fn run_variable_expression_skipped() {
        let source = "run ${my_steps}\nrun $(some_calculation)\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: false,
            require_positive_run: true,
            forbid_parent_paths: false,
        };
        let diags = check_source(source, &config);
        let lmp801: Vec<_> = diags.iter().filter(|d| d.name == "LMP801").collect();
        assert!(lmp801.is_empty());
    }

    // ---- LMP802 ----

    #[test]
    fn include_with_parent_path() {
        let source = "include ../potentials/Cu.eam\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: false,
            require_positive_run: false,
            forbid_parent_paths: true,
        };
        let diags = check_source(source, &config);
        let lmp802: Vec<_> = diags.iter().filter(|d| d.name == "LMP802").collect();
        assert_eq!(lmp802.len(), 1);
    }

    #[test]
    fn include_local_path_no_diag() {
        let source = "include potentials/Cu.eam\n";
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: false,
            require_positive_run: false,
            forbid_parent_paths: true,
        };
        let diags = check_source(source, &config);
        let lmp802: Vec<_> = diags.iter().filter(|d| d.name == "LMP802").collect();
        assert!(lmp802.is_empty());
    }

    // ---- Integration: golden fixtures ----

    #[test]
    fn valid_golden_with_matmaster() {
        let source = include_str!("../tests/fixtures/golden/valid.in");
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: true,
            require_positive_run: true,
            forbid_parent_paths: true,
        };
        let diags = check_source(source, &config);
        // valid.in has pair_style before pair_coeff and run 1000
        let lmp_codes: Vec<_> = diags.iter().map(|d| d.name).collect();
        assert!(
            !lmp_codes.contains(&"LMP800"),
            "valid.in should not trigger LMP800: {lmp_codes:?}"
        );
        assert!(
            !lmp_codes.contains(&"LMP801"),
            "valid.in should not trigger LMP801: {lmp_codes:?}"
        );
    }

    #[test]
    fn invalid_golden_with_matmaster() {
        let source = include_str!("../tests/fixtures/golden/invalid.in");
        let config = MatMasterConfig {
            enabled: true,
            check_pair_coeff_order: true,
            require_positive_run: true,
            forbid_parent_paths: true,
        };
        let diags = check_source(source, &config);
        // invalid.in has "run -10" which should trigger LMP801
        let lmp801: Vec<_> = diags.iter().filter(|d| d.name == "LMP801").collect();
        assert!(
            !lmp801.is_empty(),
            "invalid.in should trigger LMP801 for 'run -10'"
        );
    }
}
