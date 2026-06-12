//! Issue #20: lammps.files.missing_data (LAMMPS-E701)
//!
//! Error on `read_data` commands where the referenced file does not exist
//! relative to the script's directory.

use std::path::Path;

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::lints::codes::LintCode;
use crate::spans::Span;

/// A diagnostic for a `read_data` command referencing a missing file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingDataFile {
    pub path: String,
    pub span: Span,
}

impl MissingDataFile {
    /// Find all `read_data` commands whose target file does not exist.
    ///
    /// `base_dir` is the directory containing the script being analysed.
    /// When `None`, file-existence checks are skipped (no diagnostics emitted).
    pub fn find_all(ast: &Ast, base_dir: Option<&Path>) -> Vec<Self> {
        let Some(base_dir) = base_dir else {
            return Vec::new();
        };

        let mut issues = Vec::new();

        for command in &ast.commands {
            let Command::Generic(cmd) = &command else {
                continue;
            };
            if cmd.name.contents != "read_data" {
                continue;
            }
            if cmd.args.is_empty() {
                continue;
            }

            let arg = &cmd.args[0];
            let crate::ast::ArgumentKind::Word(path_str) = &arg.kind else {
                continue;
            };

            let target = base_dir.join(path_str);
            if !target.exists() {
                issues.push(Self {
                    path: path_str.clone(),
                    span: arg.span,
                });
            }
        }

        issues
    }
}

impl Issue for MissingDataFile {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            name: LintCode::MissingDataFile.label(),
            severity: Severity::Error,
            span: self.span,
            message: format!(
                "{}: missing data file `{}`",
                LintCode::MissingDataFile,
                self.path
            ),
            code: Some(LintCode::MissingDataFile.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::ts_to_ast, utils};

    #[test]
    fn missing_data_no_base_dir() {
        let source = "read_data nonexistent.data\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = MissingDataFile::find_all(&ast, None);
        assert!(issues.is_empty(), "no base_dir => no checks");
    }

    #[test]
    fn missing_data_with_missing_file() {
        let source = "read_data nonexistent.data\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let tmp = std::env::temp_dir();
        let issues = MissingDataFile::find_all(&ast, Some(&tmp));
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].path, "nonexistent.data");
        let diag = issues[0].diagnostic();
        assert_eq!(diag.severity, Severity::Error);
    }

    #[test]
    fn missing_data_with_existing_file() {
        let tmp = std::env::temp_dir();
        let file_path = tmp.join("lammps_test_data.data");
        std::fs::write(&file_path, "Header\n1 atoms\n").unwrap();

        let source = "read_data lammps_test_data.data\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = MissingDataFile::find_all(&ast, Some(&tmp));
        assert!(issues.is_empty());

        std::fs::remove_file(&file_path).ok();
    }
}
