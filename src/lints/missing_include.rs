//! Issue #19: lammps.files.missing_include (LAMMPS-E700)
//!
//! Error on `include` commands where the referenced file does not exist
//! relative to the script's directory.

use std::path::Path;

use crate::ast::{Ast, Command};
use crate::diagnostics::{Diagnostic, Issue, Severity};
use crate::lints::codes::LintCode;
use crate::spans::Span;

/// A diagnostic for an `include` command referencing a missing file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingInclude {
    pub path: String,
    pub span: Span,
}

impl MissingInclude {
    /// Find all `include` commands whose target file does not exist.
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
            if cmd.name.contents != "include" {
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

impl Issue for MissingInclude {
    fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            name: LintCode::MissingInclude.label(),
            severity: Severity::Error,
            span: self.span,
            message: format!(
                "{}: missing include file `{}`",
                LintCode::MissingInclude,
                self.path
            ),
            code: Some(LintCode::MissingInclude.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::ts_to_ast, utils};

    #[test]
    fn missing_include_no_base_dir() {
        let source = "include nonexistent.in\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = MissingInclude::find_all(&ast, None);
        assert!(issues.is_empty(), "no base_dir => no checks");
    }

    #[test]
    fn missing_include_with_missing_file() {
        let source = "include nonexistent_file.in\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let tmp = std::env::temp_dir();
        let issues = MissingInclude::find_all(&ast, Some(&tmp));
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].path, "nonexistent_file.in");
        let diag = issues[0].diagnostic();
        assert_eq!(diag.severity, Severity::Error);
    }

    #[test]
    fn missing_include_with_existing_file() {
        let tmp = std::env::temp_dir();
        let file_path = tmp.join("lammps_test_include.in");
        std::fs::write(&file_path, "units metal\n").unwrap();

        let source = "include lammps_test_include.in\n";
        let tree = utils::testing::parse(source);
        let ast = ts_to_ast(&tree, source).unwrap();
        let issues = MissingInclude::find_all(&ast, Some(&tmp));
        assert!(issues.is_empty());

        std::fs::remove_file(&file_path).ok();
    }
}
